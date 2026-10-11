use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::processing::dmc::{self, DmcColor};
use crate::processing::image_io::{self, CanvaProperties};
use crate::processing::majority_filter::majority_filter;
use crate::processing::{aggregation, color_reduction};

const DMC_CSV: &str = include_str!("../dmc_palette.csv");

#[wasm_bindgen]
pub struct Pattern {
    width: u32,
    height: u32,
    indices: Vec<u16>,
    legend_json: String,
}

#[wasm_bindgen]
impl Pattern {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn indices(&self) -> Vec<u16> {
        self.indices.clone()
    }

    pub fn legend_json(&self) -> String {
        self.legend_json.clone()
    }
}

#[wasm_bindgen]
pub fn generate_pattern(
    image_bytes: &[u8],
    unit_is_cm: bool,
    width: u32,
    height: u32,
    density_per_inch: u32,
    colors: u8,
    k: u32,
) -> Result<Pattern, JsValue> {
    run(image_bytes, unit_is_cm, width, height, density_per_inch, colors, k)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

fn run(
    image_bytes: &[u8],
    unit_is_cm: bool,
    width: u32,
    height: u32,
    density_per_inch: u32,
    colors: u8,
    k: u32,
) -> Result<Pattern, Box<dyn std::error::Error>> {
    let keep_proportions = height == 0;
    let height_opt = if keep_proportions { None } else { Some(height) };
    let canva = if unit_is_cm {
        CanvaProperties::from_cm(width, height_opt, density_per_inch)
    } else {
        CanvaProperties::from_stitches(width, height_opt)
    }
    .ok_or("nieprawidłowy rozmiar wzoru (zero w wymiarach lub gęstości)")?;

    let img = image::load_from_memory(image_bytes)?;

    let resized = image_io::resize_image(&img, canva, keep_proportions, k)?;

    let aggregated = aggregation::aggregate(&resized, k);
    let quantized = color_reduction::quantize(&aggregated, colors, None)?;

    let palette = parse_palette(DMC_CSV)?;
    let mapped = dmc::map_to_palette(&quantized, &palette);

    build_pattern(&mapped, &palette)
}

fn parse_palette(csv_text: &str) -> Result<Vec<DmcColor>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_reader(csv_text.as_bytes());
    let mut colors = Vec::new();
    for result in reader.records() {
        let record = result?;
        colors.push(DmcColor {
            dmc_code: record[0].trim().to_string(),
            color_name: record[1].trim().to_string(),
            rgb: [
                record[3].trim().parse()?,
                record[4].trim().parse()?,
                record[5].trim().parse()?,
            ],
        });
    }
    Ok(colors)
}

fn build_pattern(
    mapped: &image::RgbaImage,
    palette: &[DmcColor],
) -> Result<Pattern, Box<dyn std::error::Error>> {
    let width = mapped.width();
    let height = mapped.height();

    let by_rgb: HashMap<[u8; 3], usize> =
        palette.iter().enumerate().map(|(i, c)| (c.rgb, i)).collect();

    let mut dmc_indices: Vec<u16> = Vec::with_capacity((width * height) as usize);
    for px in mapped.pixels() {
        let dmc_idx = *by_rgb
            .get(&[px[0], px[1], px[2]])
            .ok_or("piksel poza paletą DMC (błąd w mapowaniu)")?;
        dmc_indices.push(dmc_idx as u16);
    }

    let dmc_indices = majority_filter(&dmc_indices, width as usize, height as usize);

    let mut legend_pos: HashMap<u16, u16> = HashMap::new();
    let mut legend: Vec<(usize, u32)> = Vec::new();
    let mut indices = Vec::with_capacity(dmc_indices.len());
    for &dmc_idx in &dmc_indices {
        let pos = *legend_pos.entry(dmc_idx).or_insert_with(|| {
            legend.push((dmc_idx as usize, 0));
            (legend.len() - 1) as u16
        });
        legend[pos as usize].1 += 1;
        indices.push(pos);
    }

    Ok(Pattern {
        width,
        height,
        indices,
        legend_json: legend_to_json(&legend, palette),
    })
}

fn legend_to_json(legend: &[(usize, u32)], palette: &[DmcColor]) -> String {
    let items: Vec<String> = legend
        .iter()
        .map(|&(i, count)| {
            let c = &palette[i];
            format!(
                r#"{{"code":"{}","name":"{}","rgb":[{},{},{}],"count":{}}}"#,
                escape_json(&c.dmc_code),
                escape_json(&c.color_name),
                c.rgb[0],
                c.rgb[1],
                c.rgb[2],
                count
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}