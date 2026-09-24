use image::{DynamicImage, ImageReader, RgbaImage};
use image::imageops;

pub fn load_and_resize(name: &str, width: u32, height: Option<u32>, keep_proportions: bool) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(name)?.decode()?;

    let new_height = if keep_proportions {
        (width as f32 * img.height() as f32 / img.width() as f32) as u32
    } else {
        height.ok_or("Height is required when proportions are disabled")?
    };

    Ok(imageops::resize(&img, width, new_height, imageops::FilterType::Lanczos3))
}

pub fn save(img: &RgbaImage, out_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ext = std::path::Path::new(out_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" | "bmp" => {
            DynamicImage::ImageRgba8(img.clone()).to_rgb8().save(out_path)?;
        }
        _ => {
            img.save(out_path)?;
        }
    }
    Ok(())
}