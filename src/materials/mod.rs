use crate::{color::RGBColor, objects::HitRecord, ray::Ray};

pub mod lambertarian;
pub mod metal;

pub struct MaterialScatterOutput {
    pub scattered_ray: Ray,
    pub attenuation: RGBColor,
}

pub trait Material {
    /// Calculates where and in what direction does the light bounce off the surface,
    /// and the color it contributes.
    ///
    /// ## Parameters
    /// * `incoming_ray` - the ray that hits the surface
    /// * `hit_record` - the record of the current hit
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterOutput>;
}
