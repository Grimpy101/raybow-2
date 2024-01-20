use std::sync::Arc;

use rand_xoshiro::Xoshiro256Plus;

use crate::{color::RGBColor, objects::HitRecord, ray::Ray};

use self::{dielectric::Dielectric, lambertarian::LambertarianDiffuse, metal::Metal};

pub mod dielectric;
pub mod lambertarian;
pub mod metal;

pub enum AnyMaterial {
    Metal(Metal),
    Lambertarian(LambertarianDiffuse),
    Dielectric(Dielectric),
}

impl From<Metal> for AnyMaterial {
    fn from(value: Metal) -> Self {
        Self::Metal(value)
    }
}

impl From<Metal> for Arc<AnyMaterial> {
    fn from(value: Metal) -> Self {
        Arc::new(AnyMaterial::Metal(value))
    }
}

impl From<LambertarianDiffuse> for AnyMaterial {
    fn from(value: LambertarianDiffuse) -> Self {
        Self::Lambertarian(value)
    }
}

impl From<LambertarianDiffuse> for Arc<AnyMaterial> {
    fn from(value: LambertarianDiffuse) -> Self {
        Arc::new(AnyMaterial::Lambertarian(value))
    }
}

impl From<Dielectric> for AnyMaterial {
    fn from(value: Dielectric) -> Self {
        Self::Dielectric(value)
    }
}

impl From<Dielectric> for Arc<AnyMaterial> {
    fn from(value: Dielectric) -> Self {
        Arc::new(AnyMaterial::Dielectric(value))
    }
}

impl Material for AnyMaterial {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut Xoshiro256Plus,
    ) -> Option<MaterialScatterOutput> {
        match self {
            AnyMaterial::Metal(inner) => inner.scatter(incoming_ray, hit_record, rng),
            AnyMaterial::Lambertarian(inner) => inner.scatter(incoming_ray, hit_record, rng),
            AnyMaterial::Dielectric(inner) => inner.scatter(incoming_ray, hit_record, rng),
        }
    }
}

pub struct MaterialScatterOutput {
    pub scattered_ray: Ray,
    pub attenuation: RGBColor,
}

pub trait Material {
    /// Calculates where and in what direction does the light bounce off the surface,
    /// and the color it contributes.
    ///
    /// Random number generator needs to be provided to speed up generation
    ///
    /// ## Parameters
    /// * `incoming_ray` - the ray that hits the surface
    /// * `hit_record` - the record of the current hit
    /// * `rng` - random number generator instance (thread local)
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut Xoshiro256Plus,
    ) -> Option<MaterialScatterOutput>;
}
