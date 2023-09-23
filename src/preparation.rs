use crate::{
    camera::Camera,
    color::RGBColor,
    materials::{lambertarian::LambertarianDiffuse, metal::Metal},
    math::vector3::Vector3,
    objects::sphere::Sphere,
    ray::Ray,
    rendering::renderables::Renderables,
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
    let camera = Camera::new(
        parameters.output_width,
        parameters.output_height,
        parameters.focal_length,
        Vector3::new(0.0, 0.0, 0.0),
    );

    let material_ground = LambertarianDiffuse::new_counter(RGBColor::new(0.8, 0.8, 0.0));
    let material_center = LambertarianDiffuse::new_counter(RGBColor::new(0.7, 0.3, 0.3));
    let material_left = Metal::new_counter(RGBColor::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new_counter(RGBColor::new(0.8, 0.6, 0.2), 1.0);

    let mut renderables = Renderables::new();

    let ground = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right);

    renderables.add_hittable(ground);
    renderables.add_hittable(center);
    renderables.add_hittable(left);
    renderables.add_hittable(right);

    SceneData {
        camera,
        renderables,
        background: Box::new(sky_background),
    }
}
