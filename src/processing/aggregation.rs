use image;
use image::{RgbaImage, Rgba};

fn get_segments(image: &RgbaImage, block_size: u32) -> Vec<(u32, u32, RgbaImage)> {
    let (width, height) = image.dimensions();
    let mut segments = Vec::new();

    for y in (0..height).step_by(block_size as usize) {
        if y + block_size > height { break; }
        for x in (0..width).step_by(block_size as usize) {
            if x + block_size > width { break; }
            let segment = image::imageops::crop_imm(image, x, y, block_size, block_size).to_image();
            segments.push((x, y, segment));
        }
    }
    segments
}

fn calculate_average_color(img: &RgbaImage) -> Rgba<u8> {
    let mut divisor = 0;
    let mut sum_r: u32 = 0;
    let mut sum_g: u32 = 0;
    let mut sum_b: u32 = 0;
    let mut alpha = 0;

    for pixel in img.pixels() {
        divisor += 1;
        sum_r += pixel.0[0] as u32;
        sum_g += pixel.0[1] as u32;
        sum_b += pixel.0[2] as u32;
        alpha = pixel.0[3];
    }

    let avg_r = (sum_r as f32 / divisor as f32) as u8;
    let avg_g = (sum_g as f32 / divisor as f32) as u8;
    let avg_b = (sum_b as f32 / divisor as f32) as u8;

    Rgba([avg_r, avg_g, avg_b, alpha])
}

pub fn aggregate(img: &RgbaImage, k: u32) -> RgbaImage {
    let segments: Vec<_> = get_segments(&img, k);
    let (w, h) = img.dimensions();
    let w = w / k;
    let h = h / k;
    let mut new_image = RgbaImage::new(w, h);

    for segment in segments {
        let (x, y, img) = segment;
        let x = x / k;
        let y = y / k;
        let avg_color = calculate_average_color(&img);
        new_image.put_pixel(x, y, avg_color); }

    new_image
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn output_dimensions_are_input_divided_by_k() {
        let img = RgbaImage::new(6, 4);
        let result = aggregate(&img, 2);
        assert_eq!(result.dimensions(), (3, 2));
    }

    #[test]
    fn uniform_image_keeps_its_color() {
        let color = Rgba([10, 20, 30, 255]);
        let img = RgbaImage::from_fn(4, 4, |_, _| color);
        let result = aggregate(&img, 2);
        for pixel in result.pixels() {
            assert_eq!(*pixel, color);
        }
    }

    #[test]
    fn each_cell_gets_color_of_its_own_block() {
        let img = RgbaImage::from_fn(6, 4, |x, y| {
            Rgba([(x / 2 * 50) as u8, (y / 2 * 100) as u8, 0, 255])
        });
        let result = aggregate(&img, 2);
        for cy in 0..2 {
            for cx in 0..3 {
                let expected = Rgba([(cx * 50) as u8, (cy * 100) as u8, 0, 255]);
                assert_eq!(*result.get_pixel(cx, cy), expected);
            }
        }
    }

    #[test]
    fn block_average_is_computed_per_channel() {
        let img = RgbaImage::from_fn(2, 2, |x, _| {
            if x == 0 { Rgba([0, 0, 0, 255]) } else { Rgba([255, 255, 255, 255]) }
        });
        let result = aggregate(&img, 2);
        assert_eq!(*result.get_pixel(0, 0), Rgba([127, 127, 127, 255]));
    }
}
