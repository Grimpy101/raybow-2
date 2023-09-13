use std::{error::Error, fs};

use color::RGBColor;
use output_formats::ppm::{rgb_to_ascii_ppm, rgb_to_binary_ppm};

mod color;
mod output_formats;

fn test_image(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let data_capacity = (width * height) as usize;
    let mut raw_data = Vec::with_capacity(data_capacity);

    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let color = RGBColor::new(r, g, 0.0);
            raw_data.push(color);
        }
    }

    log::info!("Writing to binary PPM...");
    let binary_ppm_data = rgb_to_binary_ppm(&raw_data, width, height)?;
    fs::write("test_binary.ppm", binary_ppm_data)?;

    log::info!("Writing to ascii PPM...");
    let ascii_ppm_data = rgb_to_ascii_ppm(&raw_data, width, height)?;
    fs::write("test_ascii.ppm", ascii_ppm_data)?;
    Ok(())
}

fn init_logger() {
    let environment = env_logger::Env::default().filter("LOG_LEVEL");
    env_logger::Builder::from_env(environment).init();
}

fn main() -> Result<(), String> {
    init_logger();

    log::info!("Starting...");
    test_image(255, 255).map_err(|err| err.to_string())?;
    log::info!("Exited");
    Ok(())
}
