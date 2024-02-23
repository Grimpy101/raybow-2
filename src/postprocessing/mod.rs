use crate::{color::RGBColor, rendering::RenderResult, Arguments};

mod gamma_correction;

pub struct PostProcessResult {
    pub width: usize,
    pub height: usize,
    pub image_data: Vec<RGBColor>,
}

/// Run postprocessing steps, such as gamma correction, etc.
///
/// ## Parameters
/// * `parameters` - application configuration arguments
/// * `render_result` - render result
pub fn postprocess(argumets: &Arguments, render_result: &RenderResult) -> PostProcessResult {
    let mut postprocessing_image_data = render_result.image_data.clone();
    if argumets.gamma_correction {
        gamma_correction::linear_to_gamma_space(&mut postprocessing_image_data);
    }

    PostProcessResult {
        width: render_result.width,
        height: render_result.height,
        image_data: postprocessing_image_data,
    }
}
