use std::{fmt::Debug, sync::Arc};

use glam::Vec3A;

use crate::{interval::Interval, materials::AnyMaterial, ray::Ray};

use self::{parallelogram::Paralellogram, sphere::Sphere};

pub mod parallelogram;
pub mod sphere;

/// A helper struct that stores information
/// about the hit, such as the location of the
/// hit, the normal and the parameter t along the ray
pub struct HitRecord {
    point: Vec3A,
    normal: Vec3A,
    t: f32,
    front_face: bool,
    material: Arc<AnyMaterial>,
}

impl HitRecord {
    pub fn new(
        point: Vec3A,
        normal: Vec3A,
        t: f32,
        front_face: bool,
        material: Arc<AnyMaterial>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    /// Sets the hit record normal vector.
    /// This is done because the stored normal always
    /// points the opposite direction of the ray,
    /// so we store additional information about the
    /// side of the object the ray hit
    ///
    /// ## Parameters
    /// * `ray`
    /// * `outward_normal` - should always be normalized!
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3A) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    /// Get current parameter along the ray
    pub fn t(&self) -> f32 {
        self.t
    }

    /// Get current normal of the hit point
    pub fn normal(&self) -> Vec3A {
        self.normal
    }

    /// Get current hit point
    pub fn point(&self) -> Vec3A {
        self.point
    }

    /// Get information if front face was hit
    pub fn front_face(&self) -> bool {
        self.front_face
    }

    /// Get current surface material
    pub fn material(&self) -> Arc<AnyMaterial> {
        self.material.clone()
    }

    /// Copy data from one HitRecord to another
    pub fn copy_from(&mut self, source: &HitRecord) {
        self.point = source.point;
        self.normal = source.normal;
        self.t = source.t;
        self.front_face = source.front_face;
        self.material = source.material.clone();
    }
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {}, {}, {} ]",
            self.point, self.normal, self.front_face
        )
    }
}

pub enum AnyHittable {
    Sphere(Sphere),
    Paralellogram(Paralellogram),
}

impl From<Sphere> for AnyHittable {
    fn from(value: Sphere) -> Self {
        Self::Sphere(value)
    }
}

impl From<Sphere> for Arc<AnyHittable> {
    fn from(value: Sphere) -> Self {
        Self::new(AnyHittable::Sphere(value))
    }
}

impl From<Paralellogram> for Arc<AnyHittable> {
    fn from(value: Paralellogram) -> Self {
        Self::new(AnyHittable::Paralellogram(value))
    }
}

impl Hittable for AnyHittable {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        match self {
            AnyHittable::Sphere(inner) => inner.hit(ray, ray_interval),
            AnyHittable::Paralellogram(inner) => inner.hit(ray, ray_interval),
        }
    }
}

/// Should be implemented on every structure
/// that can be hit by a ray
pub trait Hittable {
    /// Calculates if the ray hit the structure.
    /// It outputs true if the hit has occured,
    /// and modifies the provided `hit_record` accordingly
    ///
    /// ## Parameters
    /// * `ray` - the ray to operate with
    /// * `t_min` - the lower boundary of the path along the ray (how close to the camera we still allow the result to be)
    /// * `t_min` - the upper boundary of the path along the ray (how far from the camera we still allow the result to be)
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord>;
}
