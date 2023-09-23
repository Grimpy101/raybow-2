use std::{env, time::Instant};

use crate::postprocessing::postprocess;

mod camera;
mod color;
mod export;
mod interval;
mod materials;
mod math;
mod objects;
mod output_formats;
mod postprocessing;
mod preparation;
mod progress;
mod ray;
mod rendering;

pub struct AppParameters {
    output_path: String,
    output_width: u32,
    output_height: u32,
    focal_length: f32,
    samples_per_pixel: u32,
    steps: u32,
    gamma_correction: bool,
}

/// Initializes logging (filtered by environmental variable `LOG_LEVEL`)
fn init_logger() {
    let environment = env_logger::Env::default().filter("LOG_LEVEL");
    env_logger::Builder::from_env(environment).init();
}

/// Retrieves command line arguments and processes them into struct
fn get_parameters() -> AppParameters {
    let mut output_path = "untitled.ppm".to_string();
    let mut output_width = 256;
    let mut output_height = 256;
    let mut focal_length = 1.0;
    let mut samples_per_pixel = 1;
    let mut steps = 10;
    let mut gamma_correction = true;

    let parameters: Vec<String> = env::args().collect();
    for (i, parameter) in parameters.iter().enumerate() {
        if parameter == "--output-path" && i + 1 < parameters.len() {
            output_path = parameters[i + 1].clone();
        } else if parameter == "--output-width" && i + 1 < parameters.len() {
            output_width = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --output-width");
        } else if parameter == "--output-height" && i + 1 < parameters.len() {
            output_height = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --output-heigth");
        } else if parameter == "--focal-length" && i + 1 < parameters.len() {
            focal_length = parameters[i + 1]
                .parse::<f32>()
                .expect("Invalid parameter for --focal-length");
        } else if parameter == "--samples" && i + 1 < parameters.len() {
            samples_per_pixel = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --samples");
        } else if parameter == "--steps" && i + 1 < parameters.len() {
            steps = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --steps");
        } else if parameter == "--gamma" && i + 1 < parameters.len() {
            gamma_correction = parameters[i + 1] == "on";
        }
    }

    AppParameters {
        output_path,
        output_width,
        output_height,
        focal_length,
        samples_per_pixel,
        steps,
        gamma_correction,
    }
}

fn main() -> Result<(), String> {
    // Initialize and configure all basic stuff
    init_logger();
    let execution_time = Instant::now();
    let parameters = get_parameters();

    log::info!("Starting...");

    // ------ PREPARATION PASS ------
    log::info!("Preparing scene data...");
    let scene_data = preparation::prepare_render_data(&parameters);

    // -------- RENDER PASS --------
    log::info!("Rendering...");
    let render_result = rendering::render::render(&parameters, scene_data);

    // ------ POSTPROCESSING -------
    log::info!("Postprocessing...");
    let postprocessing_result = postprocess(&parameters, &render_result);

    // -------- EXPORT PASS --------
    log::info!("Writing to files...");
    export::export_to_file(&parameters, &postprocessing_result).map_err(|err| err.to_string())?;

    // Finalize and close everything
    let execution_duration = execution_time.elapsed();
    log::debug!("Done in {:.2?}", execution_duration);

    log::info!("Exit");
    Ok(())
}
