use crate::{interval::Interval, math::vector3::Vector3, ray::Ray};

use super::Hittable;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    /// Creates a new sphere
    ///
    /// ## Parameters
    /// * `center` - the center point of the sphere
    /// * `radius` - radius of the sphere
    pub fn new(center: Vector3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Calculates the outward normal based on provided point on the sphere
    ///
    /// ## Parameters
    /// * `point_on_sphere` - the point on the sphere to calculate normal of
    pub fn get_outward_normal(&self, point_on_sphere: Vector3) -> Vector3 {
        (point_on_sphere - self.center) / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: Interval, hit_record: &mut super::HitRecord) -> bool {
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
        let a = ray.direction().dot(&ray.direction());
        let half_b = distance.dot(&ray.direction()); // The multiplication with 2 is unnecessary (it is undone by the denominator in the term above)
        let c = distance.dot(&distance) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false; // There are no real solutions, so the ray misses the sphere
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !ray_interval.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_interval.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = self.get_outward_normal(hit_record.point);
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
