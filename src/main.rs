use std::{env, time::Instant};

mod color;
mod export;
mod math;
mod output_formats;
mod preparation;
mod rendering;

pub struct AppParameters {
    output_path: String,
    output_width: u32,
    output_height: u32,
}

/// Initialize logging (filtered by environmental variable `LOG_LEVEL`)
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
        }
    }

    AppParameters {
        output_path,
        output_width,
        output_height,
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

    // -------- EXPORT PASS --------
    log::info!("Writing to files...");
    export::export_to_file(&parameters, &render_result).map_err(|err| err.to_string())?;

    // Finalize and close everything
    let execution_duration = execution_time.elapsed();
    log::debug!("Done in {:.2?}", execution_duration);

    log::info!("Exit");
    Ok(())
}
