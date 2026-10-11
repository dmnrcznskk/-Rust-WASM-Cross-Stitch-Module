use image::DynamicImage;
use std::time::Instant;

use prototype::processing::{color_reduction, dmc, image_io};
use prototype::processing::image_io::{load_and_resize, CanvaProperties};
use prototype::processing::aggregation::aggregate;

   fn count_colors(img: &image::RgbaImage) -> usize {
       img.pixels()
           .map(|p| [p[0], p[1], p[2]])
           .collect::<std::collections::HashSet<_>>()
           .len()
   }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let img_name = &args[1];
    let target_width = args[2].parse::<u32>().unwrap();

    let full_path = format!("test_images/{img_name}");
    let out_path = format!("test_images/processed_images/{img_name}");

    let canva_prop = CanvaProperties::from_stitches(target_width, None).ok_or("Invalid canva size")?;
    let k = 8;
    let img = load_and_resize(&full_path, canva_prop, true, k)?;
    let aggregated = aggregate(&img, k);
    let quantized = color_reduction::quantize(&aggregated, 30, None)?;

    let colors = dmc::load_palette("dmc_palette.csv")?;
    let mapped = dmc::map_to_palette(&quantized, &colors);

    image_io::save(&mapped, &out_path)?;

    println!("{:?}", start.elapsed());

    println!("po quantize: {}", count_colors(&quantized));
    println!("po DMC: {}", count_colors(&mapped));

    Ok(())
}
