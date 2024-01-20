use std::rc::Rc;

use rand_xoshiro::Xoshiro256Plus;

use crate::{color::RGBColor, math::vector3::Vector3, ray::Ray};

use super::Material;

/// Dielectric material where rays bounce off the surface
/// or enter the objects refracted
///
/// Used for water, glass, ...
pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }

    pub fn new_counter(index_of_refraction: f32) -> Rc<Box<dyn Material>> {
        let dielectric = Self::new(index_of_refraction);
        let dielectric_box: Box<dyn Material> = Box::new(dielectric);
        Rc::new(dielectric_box)
    }

    /// Calculates the reflectance at the angle at which the ray hits the surface
    ///
    /// ## Parameters
    /// * `cosine` - cosine of the angle at which the ray hits the surface
    /// * `k` - ratio of refraction indices
    pub fn reflectance(cosine: f32, k: f32) -> f32 {
        // Polynomial approximation by Christophe Schlick
        let r0 = (1.0 - k) / (1.0 + k);
        let r0_2 = r0 * r0;
        r0_2 + (1.0 - r0_2) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        incoming_ray: &crate::ray::Ray,
        hit_record: &crate::objects::HitRecord,
        _rng: &mut Xoshiro256Plus,
    ) -> Option<super::MaterialScatterOutput> {
        let attenuation = RGBColor::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = incoming_ray.direction().normalize();

        let cos_theta = -unit_direction.dot(&hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // We need to check if the ray can refract! Due to Snell's law,
        // in some instances the rays cannot be refracted, so we reflect
        // them instead.
        // Theta is the angle between incoming direction and normal.
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let randomly_reflects =
            Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random();

        let direction = if cannot_refract || randomly_reflects {
            Vector3::reflect(unit_direction, hit_record.normal())
        } else {
            Vector3::refract(unit_direction, hit_record.normal(), refraction_ratio)
        };

        let scattered_ray = Ray::new(hit_record.point(), direction);
        Some(super::MaterialScatterOutput {
            scattered_ray,
            attenuation,
        })
    }
}
