use std::f32::consts::PI;

use crate::{
    camera::Camera, color::RGBColor, materials::lambertarian::LambertarianDiffuse,
    math::vector3::Vector3, objects::sphere::Sphere, ray::Ray, rendering::renderables::Renderables,
    AppParameters,
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
    let mut camera = Camera::new(
        parameters.output_width,
        parameters.output_height,
        parameters.focal_length,
        40.0,
    );
    camera.look_at(Vector3::new(0.0, 0.0, -1.0));
    camera.set_position(Vector3::new(-2.0, 2.0, 1.0));

    let mut renderables = Renderables::new();

    let r = (PI / 4.0).cos();

    let material_left = LambertarianDiffuse::new(RGBColor::new(0.0, 0.0, 1.0));
    let material_right = LambertarianDiffuse::new(RGBColor::new(1.0, 0.0, 0.0));

    let sphere_left = Sphere::new(Vector3::new(-r, 0.0, -1.5), r, material_left);
    let sphere_right = Sphere::new(Vector3::new(r, 0.0, -1.5), r, material_right);

    renderables.add_hittable(sphere_left);
    renderables.add_hittable(sphere_right);

    SceneData {
        camera,
        renderables,
        background: Box::new(sky_background),
    }
}
