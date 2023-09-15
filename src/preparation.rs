use crate::{camera::Camera, math::vector3::Vector3, AppParameters};

pub struct SceneData {
    pub camera: Camera,
}

/// Preparation stage before rendering
///
/// ## Parameters
/// * `parameters` - application parameters
pub fn prepare_render_data(parameters: &AppParameters) -> SceneData {
    let camera = Camera::new(
        parameters.output_width,
        parameters.output_height,
        parameters.focal_length,
        Vector3::new(0.0, 0.0, 0.0),
    );
    SceneData { camera }
}
