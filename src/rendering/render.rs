use crate::{
    color::RGBColor, interval::Interval, materials::Material, objects::Hittable,
    preparation::SceneData, progress::ProgressTracker, ray::Ray, AppParameters,
};

use super::RenderResult;

/// Calculates the color of the pixel
/// based on the ray hits
///
/// ## Parameters
/// * `ray`
/// * `scene_data`
fn ray_color(ray: &Ray, scene_data: &SceneData, depth: u32) -> RGBColor {
    // After some steps we conclude that the recursion
    // will not hit a light source, so we return black
    if depth == 0 {
        return RGBColor::new(0.0, 0.0, 0.0);
    }

    // The interval starts at 0.001,
    // so that we don't get shadow acne or z-fighting
    let ray_interval = Interval::new(0.001, f32::INFINITY);
    if let Some(hit_record) = scene_data.renderables.hit(ray, ray_interval) {
        if let Some(material_result) = hit_record.material().scatter(ray, &hit_record) {
            let deeper_result = ray_color(&material_result.scattered_ray, scene_data, depth - 1);
            let result = material_result.attenuation * deeper_result;
            return result;
        } else {
            return RGBColor::new(0.0, 0.0, 0.0);
        }
    }

    // If there is no hit, we calculate background
    scene_data.background.as_ref()(ray)
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
            let mut pixel_color = RGBColor::new(0.0, 0.0, 0.0);

            if parameters.samples_per_pixel == 1 {
                // We only shoot one ray through the center
                let ray = camera.get_ray_through_pixel_center(x, y);
                let result = ray_color(&ray, &scene_data, parameters.steps);
                pixel_color = result;
            } else {
                // For more rays, we do random sampling inside pixel
                for _ in 0..parameters.samples_per_pixel {
                    let ray = camera.get_random_ray_through_pixel(x, y);
                    let new_result = ray_color(&ray, &scene_data, parameters.steps);
                    pixel_color = pixel_color + new_result;
                }
            }

            // We take average of all color samples
            pixel_color = pixel_color / parameters.samples_per_pixel as f32;
            color_data.push(pixel_color);

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
