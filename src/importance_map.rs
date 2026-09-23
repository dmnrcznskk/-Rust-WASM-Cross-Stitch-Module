use image::{RgbaImage, DynamicImage, RgbImage};

pub struct ImportanceMap {
    image: RgbImage,

}

impl ImportanceMap {
    pub fn new() -> Self {
        ImportanceMap { image: RgbImage::new(0, 0) }
    }

    pub fn from(image: &DynamicImage) -> ImportanceMap {
        let img = image.to_rgb8();
        ImportanceMap { image: img, }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn new_returns_importance_map() {
        let map = ImportanceMap::new();
        assert_eq!(map.image, RgbImage::new(0, 0));
    }
    
    #[test]
    fn from_converts_to_rgb8_and_returns_instance() {
        let image = DynamicImage::ImageRgba8(RgbaImage::new(0, 0));
        let map = ImportanceMap::from(&image);
        assert_eq!(map.image, RgbImage::new(0, 0));
    }
}