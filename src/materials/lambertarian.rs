use std::rc::Rc;

use crate::{color::RGBColor, math::vector3::Vector3, ray::Ray};

use super::Material;

pub struct LambertarianDiffuse {
    albedo: RGBColor,
}

impl LambertarianDiffuse {
    /// Creates a new Lambertarian diffuse material
    ///
    /// ## Parameters
    /// * `albedo` - albedo color of the material
    pub fn new(albedo: RGBColor) -> Self {
        Self { albedo }
    }

    /// Creates a new Lambertarian diffuse material
    /// and returns reference counter of the box with this
    /// material in it. The instance is generalized to all Materials.
    ///
    /// This is a helper function in creation of the Material.
    ///
    /// ## Parameters
    /// * `albedo` - albedo color of the material
    pub fn new_counter(albedo: RGBColor) -> Rc<Box<dyn Material>> {
        let lambert = Self::new(albedo);
        let lambert_box: Box<dyn Material> = Box::new(lambert);
        Rc::new(lambert_box)
    }
}

impl Material for LambertarianDiffuse {
    fn scatter(
        &self,
        _incoming_ray: &crate::ray::Ray,
        hit_record: &crate::objects::HitRecord,
    ) -> Option<super::MaterialScatterOutput> {
        let mut scatter_direction = hit_record.normal() + Vector3::random_on_unit_sphere();
        // Handles the nasty instance where direction of the new vector
        // is (almost) the same as the normal on the surface,
        // because in that case scatter_direction would be [0.0, 0.0, 0.0]!!
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }

        let scattered_ray = Ray::new(hit_record.point(), scatter_direction);
        let attenuation = self.albedo;

        Some(super::MaterialScatterOutput {
            scattered_ray,
            attenuation,
        })
    }
}
