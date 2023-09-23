use std::{error::Error, fs};

use crate::{
    output_formats::ppm::rgb_to_binary_ppm, postprocessing::PostProcessResult, AppParameters,
};

/// Writes image data to file
///
/// ## Parameters
/// * `parameters` - global application parameters
/// * `postprocessing_result` - the result from postprocessing stage
pub fn export_to_file(
    parameters: &AppParameters,
    postprocessing_result: &PostProcessResult,
) -> Result<(), Box<dyn Error>> {
    let ppm_data = rgb_to_binary_ppm(
        &postprocessing_result.image_data,
        postprocessing_result.width,
        postprocessing_result.height,
    )?;
    fs::write(parameters.output_path.clone(), ppm_data)?;

    Ok(())
}
