use std::{env, time::Instant};

mod color;
mod export;
mod output_formats;
mod preparation;
mod rendering;

pub struct AppParameters {
    output_path: String,
    output_width: u32,
    output_height: u32,
}

fn init_logger() {
    let environment = env_logger::Env::default().filter("LOG_LEVEL");
    env_logger::Builder::from_env(environment).init();
}

/// Retrieves command line arguments and processes them into struct
fn get_parameters() -> AppParameters {
    let mut output_path = "untitled.ppm".to_string();
    let mut output_width = 256;
    let mut output_height = 256;

    let parameters: Vec<String> = env::args().collect();
    for (i, parameter) in parameters.iter().enumerate() {
        if parameter == "--output_path" && i + 1 < parameters.len() {
            output_path = parameters[i + 1].clone();
        } else if parameter == "--output_width" && i + 1 < parameters.len() {
            output_width = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --output_width");
        } else if parameter == "--output_height" && i + 1 < parameters.len() {
            output_height = parameters[i + 1]
                .parse::<u32>()
                .expect("Invalid parameter for --output_heigth");
        }
    }

    AppParameters {
        output_path,
        output_width,
        output_height,
    }
}

fn main() -> Result<(), String> {
    // Start logger (need to set `LOG_LEVEL` environmental variable upon running)
    init_logger();

    log::info!("Starting...");
    let execution_time = Instant::now();

    let parameters = get_parameters();

    // Prepare scene, objects, shaders...
    log::info!("Preparing scene data...");
    let scene_data = preparation::prepare_render_data(&parameters);

    // Actual rendering process
    log::info!("Rendering...");
    let render_result = rendering::render::render(&parameters, scene_data);

    export::export_to_file(&parameters, &render_result).map_err(|err| err.to_string())?;

    let execution_duration = execution_time.elapsed();
    log::debug!("Done in {:.2?}", execution_duration);

    log::info!("Exit");
    Ok(())
}
