use std::sync::Arc;

use glam::Vec3A;

use crate::{interval::Interval, materials::AnyMaterial, ray::Ray};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3A,
    radius: f32,
    material: Arc<AnyMaterial>,
}

impl Sphere {
    /// Creates a new sphere
    ///
    /// ## Parameters
    /// * `center` - the center point of the sphere
    /// * `radius` - radius of the sphere
    pub fn new<M>(center: Vec3A, radius: f32, material: M) -> Self
    where
        M: Into<Arc<AnyMaterial>>,
    {
        Self {
            center,
            radius,
            material: material.into(),
        }
    }

    /// Calculates the outward normal based on provided point on the sphere
    ///
    /// ## Parameters
    /// * `point_on_sphere` - the point on the sphere to calculate normal of
    pub fn get_outward_normal(&self, point_on_sphere: Vec3A) -> Vec3A {
        (point_on_sphere - self.center) / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        // To check if the ray hits,
        // we want to solve the quadratic equation
        //  -b +- sqrt(b^2 - 4ac)
        //  ---------------------
        //           2a
        // where:
        //       a = ray.direction * ray.direction
        //       b = 2 * ray.direction * (ray.origin - center)
        //       c = (ray.origin - center) * (ray.origin - center) - radius^2
        let distance = ray.origin() - self.center;
        // With optimization, we can reduce the amount the operations
        let a = ray.direction().dot(ray.direction());
        let half_b = distance.dot(ray.direction()); // The multiplication with 2 is unnecessary (it is undone by the denominator in the term above)
        let c = distance.dot(distance) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None; // There are no real solutions, so the ray misses the sphere
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !ray_interval.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let t = root;
        let outward_normal = self.get_outward_normal(point);
        let mut hit_record = HitRecord::new(point, outward_normal, t, false, self.material.clone());
        // To prevent z-fighting due to precision error, we offset hit point just a little bit
        //hit_record.point = hit_record.point + outward_normal * 0.00001;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
