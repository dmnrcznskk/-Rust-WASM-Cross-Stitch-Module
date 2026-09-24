use image::RgbaImage;
use imagequant::RGBA;
use rgb::FromSlice;

pub fn quantize(img: &RgbaImage, num_colors: u8, importance: Option<Vec<u8>>) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let width = img.width();
    let height = img.height();

    let pixels: Vec<RGBA> = img.as_raw().as_rgba().to_vec();

    let mut liq = imagequant::new();
    liq.set_max_colors(num_colors as u32)?;

    let mut liq_img = liq.new_image(pixels, width as usize, height as usize, 0.0)?;

    if let Some(map) = importance {
        liq_img.set_importance_map(map)?;
    }

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