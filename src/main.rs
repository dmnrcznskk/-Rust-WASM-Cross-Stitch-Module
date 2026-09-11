use image::{ImageReader, RgbaImage};
use image::imageops;
use imagequant::RGBA;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("image.png")?.decode()?;

    let resized = imageops::resize(&img, 250, 250, imageops::FilterType::Lanczos3);
    let width = resized.width() as usize;
    let height = resized.height() as usize;

    let pixels: Vec<RGBA> = resized
        .pixels()
        .map(|p| RGBA::new(p[0], p[1], p[2], p[3]))
        .collect();

    let mut liq = imagequant::new();
    liq.set_max_colors(16)?;

    let mut liq_img = liq.new_image(pixels, width, height, 0.0)?;

    let mut res = liq.quantize(&mut liq_img)?;
    res.set_dithering_level(1.0)?; 

    let (palette, indices) = res.remapped(&mut liq_img)?;

    let mut out_buf = Vec::with_capacity(width * height * 4);
    for idx in indices {
        let c = palette[idx as usize];
        out_buf.extend_from_slice(&[c.r, c.g, c.b, c.a]);
    }

    let out_img = RgbaImage::from_raw(width as u32, height as u32, out_buf)
        .ok_or("nieprawidłowy rozmiar bufora")?;
    out_img.save("processed.png")?;

    println!("{} x {}", width, height);
    Ok(())
}