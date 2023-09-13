use std::{error::Error, fs};

use crate::{output_formats::ppm::rgb_to_binary_ppm, rendering::RenderResult, AppParameters};

/// Writes image data to file
///
/// ## Parameters
/// * `parameters` - global application parameters
/// * `render_result` - the result from render stage
pub fn export_to_file(
    parameters: &AppParameters,
    render_result: &RenderResult,
) -> Result<(), Box<dyn Error>> {
    let ppm_data = rgb_to_binary_ppm(
        render_result.image_data(),
        render_result.width(),
        render_result.heigth(),
    )?;
    fs::write(parameters.output_path.clone(), ppm_data)?;

    Ok(())
}
