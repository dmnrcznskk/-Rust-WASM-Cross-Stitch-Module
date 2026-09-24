use image::DynamicImage;
use std::time::Instant;

mod importance_map;
mod image_io;
mod color_reduction;
mod dmc;

use importance_map::ImportanceMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let img_name = &args[1];

    let full_path = format!("test_images/{img_name}");
    let out_path = format!("test_images/processed_images/{img_name}");

    let resized = image_io::load_and_resize(&full_path, 700, None, true)?;

    let importance_map = ImportanceMap::from(&DynamicImage::ImageRgba8(resized.clone()));
    let importance = importance_map.as_u8_map();

    let quantized = color_reduction::quantize(&resized, 16, Some(importance))?;
    println!("{} x {}", quantized.width(), quantized.height());

    let colors = dmc::load_palette("dmc_palette.csv")?;
    let mapped = dmc::map_to_palette(&quantized, &colors);

    let filtered = imageproc::filter::median_filter(&mapped, 1, 1);
    image_io::save(&filtered, &out_path)?;

    println!("{:?}", start.elapsed());

    Ok(())
}