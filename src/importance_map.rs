use image::{RgbaImage, DynamicImage, RgbImage, GrayImage};
use imageproc::gradients::sobel_gradients;

pub struct ImportanceMap {
    image: RgbImage,
    edges: Option<Vec<f32>>,
}

impl ImportanceMap {
    pub fn new() -> Self {
        ImportanceMap { image: RgbImage::new(0, 0), edges: None }
    }

    pub fn from(image: &DynamicImage) -> ImportanceMap {
        let img = image.to_rgb8();
        ImportanceMap { image: img, edges: None}
    }

    fn perform_sobel(&self) -> Vec<f32> {
     let gray: GrayImage = image::imageops::grayscale(&self.image);
     let gradients = sobel_gradients(&gray);

     let img = GrayImage::from_fn(gradients.width(), gradients.height(), |x, y| {
        let val = gradients.get_pixel(x, y)[0];
        image::Luma([val.min(255) as u8])
     });

     let normalized: Vec<f32> = img.into_raw().into_iter().map(|x| (x as f32 / 255.0)).collect();
     normalized

    }

    pub fn compute(&mut self) -> Vec<f32> {
        let grayscale = self.perform_sobel();
        grayscale
    }

    pub fn image(&self) -> &RgbImage {
        &self.image
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