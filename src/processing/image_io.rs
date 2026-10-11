use image::{DynamicImage, ImageReader, RgbaImage};
use image::imageops;

#[derive(Debug)]
pub enum CanvaError {
    ZeroGridSize,
    HeightRequired,
    Image(image::ImageError)
}

// impl std::fmt::Display for CanvaError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CanvaError::ZeroGridSize => write!(f, "Grid size can't be equal to zero"),
//             CanvaError::HeightRequired => write!(f, "")
//         }
//     }
// }

/// Rozmiar wzoru podany przez użytkownika.
/// Wysokość `None` oznacza: wylicz z proporcji zdjęcia.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanvaProperties {
    /// Wymiary podane bezpośrednio w ściegach.
    Stitches { width: u32, height: Option<u32> },
    /// Wymiary w cm, gęstość kanwy w ściegach na cal (count, np. 14).
    Centimeters { width_cm: u32, height_cm: Option<u32>, density_per_inch: u32 },
}

impl CanvaProperties {
    pub fn from_stitches(width: u32, height: Option<u32>) -> Option<Self> {
        if width == 0 || height == Some(0) { return None; }

        Some(CanvaProperties::Stitches { width, height })
    }

    pub fn from_cm(width_cm: u32, height_cm: Option<u32>, density_per_inch: u32) -> Option<Self> {
        if width_cm == 0 || height_cm == Some(0) || density_per_inch == 0 { return None; }

        Some(CanvaProperties::Centimeters { width_cm, height_cm, density_per_inch })
    }

    pub fn grid_width(&self) -> u32 {
        match *self {
            CanvaProperties::Stitches { width, .. } => width,
            CanvaProperties::Centimeters { width_cm, density_per_inch, .. } => {
                cm_to_stitches(width_cm, density_per_inch)
            }
        }
    }

    pub fn grid_height(&self) -> Option<u32> {
        match *self {
            CanvaProperties::Stitches { height, .. } => height,
            CanvaProperties::Centimeters { height_cm, density_per_inch, .. } => {
                height_cm.map(|h| cm_to_stitches(h, density_per_inch))
            }
        }
    }
}

fn cm_to_stitches(cm: u32, density_per_inch: u32) -> u32 {
    let density_per_cm = density_per_inch as f32 / 2.54;
    (cm as f32 * density_per_cm).round() as u32
}

pub fn resize_image(img: &DynamicImage, canva_prop: CanvaProperties, keep_proportions: bool, k: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    if k == 0 {
        return Err("k must be greater than 0".into());
    }

    let grid_width = canva_prop.grid_width();

    let grid_height = if keep_proportions {
        (grid_width as f32 * img.height() as f32 / img.width() as f32).round() as u32
    } else {
        canva_prop.grid_height().ok_or("Height is required when proportions are disabled")?
    };

    if grid_width == 0 || grid_height == 0 {
        return Err("Grid size must be at least 1x1 stitch".into());
    }

    Ok(imageops::resize(img, grid_width * k, grid_height * k, imageops::FilterType::Lanczos3))
}

pub fn load_and_resize(name: &str, canva_prop: CanvaProperties, keep_proportions: bool, k: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(name)?.decode()?;
    resize_image(&img, canva_prop, keep_proportions, k)
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
    fn cm_width_is_converted_by_density() {
        let canva = CanvaProperties::from_cm(10, None, 14).unwrap();
        assert_eq!(canva.grid_width(), 55);
    }

    #[test]
    fn cm_height_is_converted_by_density() {
        let canva = CanvaProperties::from_cm(10, Some(5), 14).unwrap();
        assert_eq!(canva.grid_height(), Some(28));
    }

    #[test]
    fn cm_height_none_stays_none() {
        let canva = CanvaProperties::from_cm(10, None, 14).unwrap();
        assert_eq!(canva.grid_height(), None);
    }

    #[test]
    fn stitches_are_used_directly() {
        let canva = CanvaProperties::from_stitches(55, Some(40)).unwrap();
        assert_eq!(canva.grid_width(), 55);
        assert_eq!(canva.grid_height(), Some(40));
    }

    #[test]
    fn stitches_height_none_stays_none() {
        let canva = CanvaProperties::from_stitches(55, None).unwrap();
        assert_eq!(canva.grid_height(), None);
    }

    #[test]
    fn zero_values_are_rejected() {
        assert_eq!(CanvaProperties::from_cm(0, None, 14), None);
        assert_eq!(CanvaProperties::from_cm(10, Some(0), 14), None);
        assert_eq!(CanvaProperties::from_cm(10, None, 0), None);
        assert_eq!(CanvaProperties::from_stitches(0, None), None);
        assert_eq!(CanvaProperties::from_stitches(55, Some(0)), None);
    }
}