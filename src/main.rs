use std::time::Instant;

use argh::FromArgs;

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
#[derive(FromArgs)]
/// # Raybow 2
/// A little raytracer
pub struct Arguments {
    /// output path without final extension [String]
    #[argh(option, default = "String::from(\"untitled\")", short = 'o')]
    output_path: String,
    /// output image width [u32]
    #[argh(option, default = "256")]
    output_width: usize,
    /// output image height [u32]
    #[argh(option, default = "256")]
    output_height: usize,
    /// focal length of the camera [f32]
    #[argh(option, default = "1.0")]
    focal_length: f32,
    /// amount of rays to send from each pixel [u32] (more means better quality and anti-aliasing, but is slower)
    #[argh(option, default = "1")]
    samples_per_pixel: usize,
    /// amount of bounces each ray makes [u32] (more means more realism and better quality, but is slower)
    #[argh(option, default = "10")]
    steps: usize,
    /// whether to apply gamma correction to the final image
    #[argh(switch)]
    gamma_correction: bool,
    /// show verbose messages about program execution
    #[argh(switch, short = 'v')]
    verbose: bool,
}

/// Initializes logging (filtered by environmental variable `LOG_LEVEL`)
fn init_logger(is_verbose: bool) {
    //let environment = env_logger::Env::default().filter("LOG_LEVEL");
    //env_logger::Builder::from_env(environment).init();
    let mut builder = env_logger::Builder::new();
    if is_verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Warn);
    }
    builder.init();
}

fn main() -> Result<(), String> {
    // Initialize and configure all basic stuff
    let arguments: Arguments = argh::from_env();
    init_logger(arguments.verbose);

    let execution_time = Instant::now();

    log::info!("Starting...");

    // ------ PREPARATION PASS ------ //
    log::info!("Preparing scene data...");
    let scene_data = preparation::prepare_render_data(&arguments);

    // -------- RENDER PASS -------- //
    log::info!("Rendering...");
    let render_result = rendering::render::render(&arguments, scene_data);

    // ------ POSTPROCESSING ------- //
    log::info!("Postprocessing...");
    let postprocessing_result = postprocess(&arguments, &render_result);

    // -------- EXPORT PASS -------- //
    log::info!("Writing to files...");
    export::export_to_file(&arguments, &postprocessing_result).map_err(|err| err.to_string())?;

    // Finalize and close everything
    let execution_duration = execution_time.elapsed();
    log::debug!("Done in {:.2?}", execution_duration);

    log::info!("Exit");
    Ok(())
}
