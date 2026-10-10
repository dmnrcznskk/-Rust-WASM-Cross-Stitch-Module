use image::DynamicImage;
use std::time::Instant;

mod importance_map;
mod image_io;
mod color_reduction;
mod dmc;
mod aggregation;

use importance_map::ImportanceMap;
use image_io::{load_and_resize, CanvaProperties};
use aggregation::aggregate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let img_name = &args[1];
    let target_width = args[2].parse::<u32>().unwrap();

    let full_path = format!("test_images/{img_name}");
    let out_path = format!("test_images/processed_images/{img_name}");

    let canva_prop = CanvaProperties::new(target_width, None, 14).ok_or("Zupa")?;
    let k = 8;
    let img = load_and_resize(&full_path, canva_prop, true, k)?;
    let aggregated = aggregate(&img, k);
    let quantized = color_reduction::quantize(&aggregated, 16, None)?;
    println!("{} x {}", quantized.width(), quantized.height());

    let colors = dmc::load_palette("dmc_palette.csv")?;
    let mapped = dmc::map_to_palette(&quantized, &colors);

    let filtered = imageproc::filter::median_filter(&mapped, 1, 1);
    image_io::save(&filtered, &out_path)?;

    println!("{:?}", start.elapsed());

    Ok(())
}
