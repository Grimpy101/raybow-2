use std::f32::consts::PI;

use glam::Vec3A;

use crate::{
    camera::Camera,
    color::RGBColor,
    materials::lambertarian::LambertarianDiffuse,
    objects::{parallelogram::Parallelogram, sphere::Sphere},
    ray::Ray,
    rendering::renderables::Renderables,
    Arguments,
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
pub fn prepare_render_data(arguments: &Arguments) -> SceneData {
    let mut camera = Camera::default();
    camera.set_width(arguments.output_width);
    camera.set_height(arguments.output_height);
    camera.set_vertical_fov(arguments.fov);
    camera.set_defocus(arguments.dof_distance, arguments.dof_size);
    camera.look_at(Vec3A::new(0.0, 0.0, -1.0));
    camera.set_position(Vec3A::new(-3.0, 3.0, 1.0));

    let mut renderables = Renderables::new();

    let r = (PI / 4.0).cos();

    let material_left = LambertarianDiffuse::new(RGBColor::new(0.0, 0.0, 1.0));
    let material_right = LambertarianDiffuse::new(RGBColor::new(1.0, 0.0, 0.0));

    let sphere_left = Sphere::new((-r, 0.0, -1.0).into(), r, material_left);
    let sphere_right = Sphere::new((r, 0.0, -1.0).into(), r, material_right);

    let material_plane = LambertarianDiffuse::new(RGBColor::new(0.0, 1.0, 0.0));
    let plane = Parallelogram::new(
        (-1.0, 0.0, -1.0).into(),
        (1.0, 0.0, 0.0).into(),
        (0.0, 0.0, 1.0).into(),
        material_plane,
    );

    renderables.add_hittable(sphere_left);
    renderables.add_hittable(sphere_right);
    renderables.add_hittable(plane);

    SceneData {
        camera,
        renderables,
        background: Box::new(sky_background),
    }
}
