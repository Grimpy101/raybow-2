use crate::{color::RGBColor, preparation::SceneData, AppParameters};

use super::RenderResult;

/// The main rendering process
///
/// ## Parameters
/// * `parameters` - global application parameters
/// * `scene_data` - scene data to render
pub fn render(parameters: &AppParameters, _scene_data: SceneData) -> RenderResult {
    let width = parameters.output_width;
    let height = parameters.output_height;

    // For progress tracking
    let tenth_of_work = ((width * height) as f32 * 0.1) as u32;
    let mut absolute_progress = 0;
    let mut relative_progress = 0;

    let mut color_data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let color = RGBColor::new(r, g, 0.0);
            color_data.push(color);

            // Clumsy code to track progress
            if absolute_progress % tenth_of_work == 0 {
                log::info!("Rendering {}% complete", relative_progress);
                relative_progress += 10;
            }
            absolute_progress += 1;
        }
    }

    RenderResult {
        width,
        height,
        image_data: color_data,
    }
}
