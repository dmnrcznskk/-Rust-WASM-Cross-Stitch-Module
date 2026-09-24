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

    // fn get_segments(&self, block_size: u32) -> Vec<(u32, u32, RgbImage)> {
    //     let (width, height) = &self.image.dimensions();
    //     let mut segments = Vec::new();

    //     for y in (0..height).step_by(block_size as usize) {
    //         if y + block_size > height { break; }
    //         for x in (0..width).step_by(block_size as usize) {
    //             if x + block_size > width { break; }
    //             let segment = image::imageops::crop_imm(&self.image, x, y, block_size, block_size).to_image();
    //             segments.push((x, y, segment));
    //         }
    //     }
    //     segments
    // }
 

    pub fn compute(&mut self) -> Vec<f32> {
        let grayscale = self.perform_sobel();
        grayscale
    }

    pub fn as_u8_map(&self) -> Vec<u8> {
    let gray = image::imageops::grayscale(&self.image);
    let gradients = sobel_gradients(&gray);
    gradients.as_raw().iter().map(|&v| v.min(255) as u8).collect()
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