use std::rc::Rc;

use crate::{color::RGBColor, math::vector3::Vector3, ray::Ray};

use super::Material;

pub struct Metal {
    albedo: RGBColor, // Color of the surface
    roughness: f32,   // How rough (unclear) is the surface
}

impl Metal {
    /// Creates a new Metal material
    ///
    /// ## Parameters
    /// * `albedo` - albedo color of the material
    /// * `roughness` - 0.0 means completely clear material, 1.0 means rough material
    pub fn new(color: RGBColor, roughness: f32) -> Self {
        Self {
            albedo: color,
            roughness,
        }
    }

    /// Creates a new Metal material
    /// and returns reference counter of the box with this
    /// material in it. The instance is generalized to all Materials.
    ///
    /// This is a helper function in creation of the Material.
    ///
    /// ## Parameters
    /// * `albedo` - albedo color of the material
    /// * `roughness` - 0.0 means completely clear material, 1.0 means rough material
    pub fn new_counter(color: RGBColor, roughness: f32) -> Rc<Box<dyn Material>> {
        let metal = Metal::new(color, roughness);
        let metal_box: Box<dyn Material> = Box::new(metal);
        Rc::new(metal_box)
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: &crate::ray::Ray,
        hit_record: &crate::objects::HitRecord,
    ) -> Option<super::MaterialScatterOutput> {
        // We reflect the ray over the normal so the bounce is clean.
        // We achieve roughness by shifting scatter direction by a random unit vector, scaled by roughness parameter
        let reflected = Vector3::reflect(incoming_ray.direction().normalize(), hit_record.normal())
            + self.roughness * Vector3::random_on_unit_sphere();
        let scattered_ray = Ray::new(hit_record.point(), reflected);
        let attenuation = self.albedo;
        if scattered_ray.direction().dot(&hit_record.normal()) > 0.0 {
            Some(super::MaterialScatterOutput {
                scattered_ray,
                attenuation,
            })
        } else {
            None
        }
    }
}
