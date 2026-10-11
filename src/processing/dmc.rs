use image::RgbaImage;
use quantize_palette::palette::{Metric, Palette};
use quantize_palette::quantize::{self, AlphaMode, Dither};

#[derive(Debug)]
pub struct DmcColor {
    pub dmc_code: String,
    pub color_name: String,
    pub rgb: [u8; 3],
}

pub fn load_palette(path: &str) -> Result<Vec<DmcColor>, Box<dyn std::error::Error>> {
    let mut colors = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let dmc_code = record[0].trim().to_string();
        let color_name = record[1].trim().to_string();
        let r: u8 = record[3].trim().parse()?;
        let g: u8 = record[4].trim().parse()?;
        let b: u8 = record[5].trim().parse()?;

        colors.push(DmcColor { dmc_code, color_name, rgb: [r, g, b] });
    }

    Ok(colors)
}

pub fn map_to_palette(img: &RgbaImage, colors: &[DmcColor]) -> RgbaImage {
    let for_palette: Vec<[u8; 3]> = colors.iter().map(|c| c.rgb).collect();
    let palette = Palette::from_colors(for_palette);

    quantize::quantize(img, &palette, Metric::Oklab, AlphaMode::Binarize(128), Dither::None)
}