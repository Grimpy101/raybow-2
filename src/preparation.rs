use crate::{
    camera::Camera, math::vector3::Vector3, objects::sphere::Sphere,
    rendering::renderables::Renderables, AppParameters,
};

pub struct SceneData {
    pub camera: Camera,
    pub renderables: Renderables,
}

/// Preparation stage before rendering
///
/// Prepares all renderables, constructs the scene,
/// and configures the camera
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
    let mut renderables = Renderables::new();
    let small_sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let big_sphere = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    renderables.add_hittable(small_sphere);
    renderables.add_hittable(big_sphere);
    SceneData {
        camera,
        renderables,
    }
}
