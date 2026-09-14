use image::{ImageReader, RgbaImage};
use image::imageops;
use imagequant::RGBA;
use rgb::FromSlice;

fn load_and_resize_img(name: &str, width: u32, height: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(name)?.decode()?;
    let resized = imageops::resize(&img, width, height, imageops::FilterType::Lanczos3);
    Ok(resized)
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resized = load_and_resize_img("image.png", 250, 250)?;
    let quantized = quantize_image(&resized, 16)?;

    quantized.save("processed.png")?;

    println!("{} x {}", quantized.width(), quantized.height());
    Ok(())
}