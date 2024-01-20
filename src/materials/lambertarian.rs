use std::sync::Arc;

use rand_xoshiro::Xoshiro256Plus;

use crate::{color::RGBColor, math::vector3::Vector3, ray::Ray};

use super::Material;

/// Lambertarian diffuse material
///
/// Works by sending rays in random directions from point of contact.
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
    pub fn new_counter(albedo: RGBColor) -> Arc<Self> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for LambertarianDiffuse {
    fn scatter(
        &self,
        _incoming_ray: &crate::ray::Ray,
        hit_record: &crate::objects::HitRecord,
        rng: &mut Xoshiro256Plus,
    ) -> Option<super::MaterialScatterOutput> {
        let random_unit_vector = Vector3::random_on_unit_sphere(rng);
        let scatter_direction = hit_record.normal() + random_unit_vector;
        // Handles the nasty instance where direction of the new vector
        // is (almost) the same as the normal on the surface,
        // because in that case scatter_direction would be [0.0, 0.0, 0.0]!!
        // TODO: Or does it? Produces weird artefacts...
        if scatter_direction.near_zero() {
            //scatter_direction = hit_record.normal();
        }

        if scatter_direction.is_invalid() {
            log::debug!("{}, {}", hit_record.normal(), random_unit_vector);
        }

        let scattered_ray = Ray::new(hit_record.point(), scatter_direction);
        let attenuation = self.albedo;

        Some(super::MaterialScatterOutput {
            scattered_ray,
            attenuation,
        })
    }
}
