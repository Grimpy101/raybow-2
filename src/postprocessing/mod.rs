use crate::{color::RGBColor, rendering::RenderResult, AppParameters};

mod gamma_correction;

pub struct PostProcessResult {
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<RGBColor>,
}

pub fn postprocess(parameters: &AppParameters, render_result: &RenderResult) -> PostProcessResult {
    let mut postprocessing_image_data = render_result.image_data.clone();
    if parameters.gamma_correction {
        gamma_correction::linear_to_gamma_space(&mut postprocessing_image_data);
    }

    PostProcessResult {
        width: render_result.width,
        height: render_result.height,
        image_data: postprocessing_image_data,
    }
}
