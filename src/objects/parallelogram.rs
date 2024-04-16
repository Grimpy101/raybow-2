use std::sync::Arc;

use glam::Vec3A;

use crate::{interval::Interval, materials::AnyMaterial, ray::Ray};

use super::{HitRecord, Hittable};

/// The parallelogram is defined by a bottom left point
/// and two vectors pointing from bottom left point
/// to the other three points
pub struct Paralellogram {
    bottom_left_point: Vec3A,
    up: Vec3A,
    right: Vec3A,
    normal: Vec3A,
    plane_parameter: f32,
    w: Vec3A,

    material: Arc<AnyMaterial>,
}

impl Paralellogram {
    pub fn new<M>(bottom_left_point: Vec3A, up: Vec3A, right: Vec3A, material: M) -> Self
    where
        M: Into<Arc<AnyMaterial>>,
    {
        let n = right.cross(up);
        let normal = n.normalize();
        let plane_parameter = normal.dot(bottom_left_point);
        let w = n / n.dot(n);

        Self {
            bottom_left_point,
            up,
            right,
            normal,
            plane_parameter,
            w,
            material: material.into(),
        }
    }
}

impl Hittable for Paralellogram {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction());

        if denominator.abs() < f32::EPSILON {
            return None;
        }

        // We first test if the ray intersects the plane on which the parallelogram is located.
        let numerator = self.plane_parameter - self.normal.dot(ray.origin());
        let t = numerator / denominator;
        if !ray_interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        // We express intersection with `up` and `right` ray.
        // If coefficients are between 0.0 and 1.0, the intersection is on the parallelogram.
        let p = intersection - self.bottom_left_point;
        let a = self.w.dot(p.cross(self.up));
        let b = self.w.dot(self.right.cross(p));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }

        let mut hit_record =
            HitRecord::new(intersection, self.normal, t, true, self.material.clone());
        hit_record.set_face_normal(ray, self.normal);
        Some(hit_record)
    }
}
