use image::{DynamicImage, ImageReader, RgbaImage, RgbImage};
use image::imageops;

#[derive(Debug)]
pub enum CanvaError {
    ZeroGridSize,
    HeightRequired,
    Image(image::ImageError)
}

// impl std::fmt::Display for CanvaError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>') -> std::fmt::Result {
//         match self {
//             CanvaError::ZeroGridSize => write!(f, "Grid size can't be equal to zero"),
//             CanvaError::HeightRequired => write!(f, "")
//         }
//     }
// }
//


#[derive(Debug, PartialEq)]
pub enum CanvaProperties {
    Stitches { width: u32, height: Option<u32> },
    Centimeters { width: u32, height: Option<u32>, density_inches: u32 },
}

impl CanvaProperties {
    pub fn from_stitches(w: u32, h: Option<u32>) -> Option<Self> {
        if w == 0 || h == Some(0) { return None; }
        Some(CanvaProperties::Stitches { width: w, height: h })
    }

    pub fn from_cm(w: u32, h: Option<u32>, density_inches: u32) -> Option<Self> {
        if w == 0 || h == Some(0) || density_inches == 0 { return None; }
        Some(CanvaProperties::Centimeters { width: w, height: h, density_inches })
    }

    pub fn grid_width(&self) -> u32 {
        match *self {
            CanvaProperties::Stitches { width, .. } => width,
            CanvaProperties::Centimeters { width, density_inches, .. } => {
                cm_to_stitches(width, density_inches)
            }
        }
    }

    pub fn grid_height(&self) -> Option<u32> {
        match *self {
            CanvaProperties::Stitches { height, .. } => height,
            CanvaProperties::Centimeters { height, density_inches, .. } => {
                height.map(|h| cm_to_stitches(h, density_inches))
            }
        }
    }
}
 
fn cm_to_stitches(cm: u32, density_per_inch: u32) -> u32 {
    let density_per_cm = density_per_inch as f32 / 2.54;
    (cm as f32 * density_per_cm).round() as u32
}
 
pub fn load_and_resize(name: &str, canva_prop: CanvaProperties, keep_proportions: bool, k: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    if k == 0 {
        return Err("k must be greater than 0".into());
    }
 
    let img = ImageReader::open(name)?.decode()?;
 
    let grid_width = canva_prop.grid_width();
 
    let grid_height = if keep_proportions {
        (grid_width as f32 * img.height() as f32 / img.width() as f32).round() as u32
    } else {
        canva_prop.grid_height().ok_or("Height is required when proportions are disabled")?
    };
 
    if grid_width == 0 || grid_height == 0 {
        return Err("Grid size must be at least 1x1 stitch".into());
    }
 
    Ok(imageops::resize(&img, grid_width * k, grid_height * k, imageops::FilterType::Lanczos3))
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

    #[test]
    fn test_grid_height() {
        let canva = CanvaProperties::new(10, Some(5), 14).expect("Dupa");
        let gh = canva.grid_height().unwrap();
        assert_eq!(gh, 28)
    }

    #[test]
    fn test_grid_height_when_is_none() {
        let canva = CanvaProperties::new(10, None, 14).expect("Dupa");
        assert_eq!(canva.grid_height(), None)
    }
}
