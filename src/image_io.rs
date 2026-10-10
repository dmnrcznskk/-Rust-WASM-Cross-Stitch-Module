use image::{DynamicImage, ImageReader, RgbaImage};
use image::imageops;

struct CanvaProperties {
    width_cm: u32,
    height_cm: Option<u32>,
    density_per_inch: u32,
}

impl CanvaProperties {
    pub fn new(width_cm: u32, height_cm: Option<u32>, density_per_inch: u32) -> Option<Self> {
        if density_per_inch == 0 { return None; }

        Some( CanvaProperties{ width_cm, height_cm, density_per_inch })
    }

    pub fn width_cm(&self) -> u32 { self.width_cm }
    pub fn height_cm(&self) -> Option<u32> { self.height_cm }
    pub fn density_per_inch(&self) -> u32 { self.density_per_inch }

    pub fn density_per_cm(&self) -> f32 { 
        self.density_per_inch as f32 / 2.54
    }

    pub fn grid_width(&self) -> u32 {
        ((self.width_cm as f32 * self.density_per_cm()).round()) as u32
    }

    pub fn grid_height(&self) -> Option<u32> {
        let h = self.height_cm?;
        Some((h as f32 * self.density_per_cm()).round() as u32)
    }
}

pub fn load_and_resize(name: &str, canva_prop: CanvaProperties, keep_proportions: bool) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(name)?.decode()?;

    let grid_width = canva_prop.grid_width();

    let grid_height = if keep_proportions {
        (grid_width as f32 * img.height() as f32 / img.width() as f32) as u32
    } else {
        canva_prop.grid_height().ok_or("Height is required when proportions are disabled")?
    };

    Ok(imageops::resize(&img, grid_width, grid_height, imageops::FilterType::Lanczos3))
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid_width() {
        let canva = CanvaProperties::new(10, None, 14).expect("Dupa");
        assert_eq!(canva.grid_width(), 55);
    }
}