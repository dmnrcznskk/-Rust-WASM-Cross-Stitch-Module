use image::{ImageReader, RgbaImage, ImageBuffer};
use image::imageops;
use imagequant::RGBA;
use rgb::FromSlice;
use quantize_palette::palette::{Metric, Palette};
use quantize_palette::quantize::{quantize, AlphaMode, Dither};


fn load_and_resize_img(name: &str, width: u32, height: Option<u32>, keep_proportions: bool) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(name)?.decode()?;

    let new_height = if keep_proportions {
        (width as f32 * img.height() as f32 / img.width() as f32) as u32
    } else {
        height.ok_or("Height is required when proportions are disabled")?
    };
    
    let resized = imageops::resize(&img, width, new_height, imageops::FilterType::Lanczos3);
    Ok(resized)
}

//fn apply_median_filter()

fn quantize_image(img: &RgbaImage, num_colors: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let width = img.width();
    let height = img.height();

    let pixels: Vec<RGBA> = img.as_raw().as_rgba().to_vec();

    let mut liq = imagequant::new();
    liq.set_max_colors(num_colors as u32)?;

    let mut liq_img = liq.new_image(pixels, width as usize, height as usize, 0.0)?;

    let mut res = liq.quantize(&mut liq_img)?;
    res.set_dithering_level(1.0)?;

    let (palette, indices) = res.remapped(&mut liq_img)?;

    let raw_buf: Vec<u8> = indices
        .iter()
        .flat_map(|&idx| {
            let c = palette[idx as usize];
            [c.r, c.g, c.b, c.a]
        })
        .collect();

    RgbaImage::from_raw(width, height, raw_buf)
        .ok_or_else(|| "Wrong buffer size".into())
}

#[derive(Debug)]
struct DmcColor {
    dmc_code: String,
    color_name: String,
    rgb: [u8; 3],
}

fn colors_csv_to_vec(path: &str) -> Result<Vec<DmcColor>, Box<dyn std::error::Error>> {
    let mut colors = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let dmc_code = record[0].trim().to_string();
        let color_name = record[1].trim().to_string();
        let r: u8 = record[3].trim().parse()?;
        let g: u8 = record[4].trim().parse()?;
        let b: u8 = record[5].trim().parse()?;

        let color = DmcColor { dmc_code, color_name, rgb: [r, g, b]};
        colors.push(color);
    }

    Ok(colors)
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let img_name = &args[1];

    let full_path = format!("test_images/{img_name}");
    let out_path = format!("test_images/processed_images/{img_name}");

    let resized = load_and_resize_img(&full_path, 250, None, true)?;
    let quantized = quantize_image(&resized, 32)?;

    println!("{} x {}", quantized.width(), quantized.height());

    let colors = colors_csv_to_vec("dmc_palette.csv")?;

    let mut for_palette = Vec::new();
    for color in &colors {
        let rgb = color.rgb;
        for_palette.push(rgb);
    }


    let palette = Palette::from_colors(for_palette);

    let out = quantize(&quantized, &palette, Metric::Oklab, AlphaMode::Binarize(128), Dither::None);

    let filtered = imageproc::filter::median_filter(&out, 1, 1);

    filtered.save(&out_path)?;

    Ok(())
}
