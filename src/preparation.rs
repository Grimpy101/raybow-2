use crate::{
    camera::Camera, color::RGBColor, math::vector3::Vector3, objects::sphere::Sphere, ray::Ray,
    rendering::renderables::Renderables, AppParameters,
};

pub struct SceneData {
    pub camera: Camera,
    pub renderables: Renderables,
    pub background: Box<dyn Fn(&Ray) -> RGBColor>,
}

/// Calculates sky background color
pub fn sky_background(ray: &Ray) -> RGBColor {
    let unit_direction = ray.direction().normalize();
    let parameter = 0.5 * (unit_direction.y + 1.0);
    let start_color = RGBColor::new(1.0, 1.0, 1.0); // White
    let end_color = RGBColor::new(0.5, 0.7, 1.0); // Blue
    RGBColor::lerp(start_color, end_color, parameter) // We interpolate between white and blue based on vertical direction of the ray
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
        background: Box::new(sky_background),
    }
}
