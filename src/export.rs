use std::{error::Error, fs};

use crate::{
    output_formats::ppm::rgb_to_binary_ppm, postprocessing::PostProcessResult, AppParameters,
};

/// Writes image data to file
///
/// ## Parameters
/// * `parameters` - global application parameters
/// * `render_result` - the result from render stage
pub fn export_to_file(
    parameters: &AppParameters,
    postprocess_result: &PostProcessResult,
) -> Result<(), Box<dyn Error>> {
    let ppm_data = rgb_to_binary_ppm(
        &postprocess_result.image_data,
        postprocess_result.width,
        postprocess_result.height,
    )?;
    fs::write(parameters.output_path.clone(), ppm_data)?;

    Ok(())
}
