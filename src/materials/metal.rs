use std::rc::Rc;

use rand_xoshiro::Xoshiro256Plus;

use crate::{
    color::RGBColor,
    math::{random_vec3_on_unit_sphere, reflect_vec3},
    objects::HitRecord,
    ray::Ray,
};

use super::{Material, MaterialScatterOutput};

/// Metallic material
///
/// Works by reflecting incoming rays over the normal in the contact point of the surface.
/// To control roughness (clearness) of the material, a roughness parameter displaces
/// reflected rays to create a hazy reflections.
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
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut Xoshiro256Plus,
    ) -> Option<MaterialScatterOutput> {
        // We reflect the ray over the normal so the bounce is clean.
        // We achieve roughness by shifting scatter direction by a random unit vector, scaled by roughness parameter
        let reflected = reflect_vec3(incoming_ray.direction().normalize(), hit_record.normal())
            + self.roughness * random_vec3_on_unit_sphere(rng);
        let scattered_ray = Ray::new(hit_record.point(), reflected);
        let attenuation = self.albedo;
        if scattered_ray.direction().dot(hit_record.normal()) > 0.0 {
            Some(super::MaterialScatterOutput {
                scattered_ray,
                attenuation,
            })
        } else {
            None
        }
    }
}
