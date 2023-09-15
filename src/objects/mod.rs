use crate::{math::vector3::Vector3, ray::Ray};

pub mod sphere;

/// A helper struct that stores information
/// about the hit, such as the location of the
/// hit, the normal and the parameter t along the ray
pub struct HitRecord {
    point: Vector3,
    normal: Vector3,
    t: f32,
    front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: -1.0,
            front_face: true,
        }
    }
}

impl HitRecord {
    /// Sets the hit record normal vector.
    /// This is done because the stored normal always
    /// points the opposite direction of the ray,
    /// so we store additional information about the
    /// side of the object the ray hit
    ///
    /// ## Parameters
    /// * `ray`
    /// * `outward_normal` - should always be normalized!
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
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
    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    /// Copy data from one HitRecord to another
    pub fn copy_from(&mut self, source: &HitRecord) {
        self.point = source.point;
        self.normal = source.normal;
        self.t = source.t;
        self.front_face = source.front_face;
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
