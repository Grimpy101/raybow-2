use crate::{
    color::RGBColor,
    interval::Interval,
    objects::{HitRecord, Hittable},
    preparation::SceneData,
    progress::ProgressTracker,
    ray::Ray,
    AppParameters,
};

use super::RenderResult;

/// Calculates the color of the pixel
/// based on the ray hits
///
/// ## Parameters
/// * `ray`
fn ray_color(ray: &Ray, scene_data: &SceneData) -> RGBColor {
    let mut hit_record = HitRecord::default();
    let ray_interval = Interval::new(0.0, f32::INFINITY);
    if scene_data
        .renderables
        .hit(ray, ray_interval, &mut hit_record)
    {
        let normal = hit_record.normal();
        return 0.5 * RGBColor::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = ray.direction().normalize();
    let parameter = 0.5 * (unit_direction.y + 1.0);
    let start_color = RGBColor::new(1.0, 1.0, 1.0);
    let end_color = RGBColor::new(0.5, 0.7, 1.0);
    RGBColor::lerp(start_color, end_color, parameter)
}

/// The main rendering process
///
/// ## Parameters
/// * `parameters` - global application parameters
/// * `scene_data` - scene data to render
pub fn render(parameters: &AppParameters, scene_data: SceneData) -> RenderResult {
    let width = parameters.output_width;
    let height = parameters.output_height;

    let camera = &scene_data.camera;

    // For progress tracking
    let mut progress_tracker = ProgressTracker::new(0.0, (width * height) as f32, 1.0, 0.1);

    let mut color_data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let pixel_center = camera.get_pixel_center(x, y);
            let ray_direction = pixel_center - camera.origin();

            let ray = Ray::new(camera.origin(), ray_direction);

            let color = ray_color(&ray, &scene_data);
            color_data.push(color);

            if let Some(progress) = progress_tracker.increment() {
                log::debug!(" Render on {:.0}%", progress * 100.0)
            };
        }
    }

    RenderResult {
        width,
        height,
        image_data: color_data,
    }
}
