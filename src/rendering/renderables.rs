use std::sync::Arc;

use crate::{
    interval::Interval,
    objects::{AnyHittable, HitRecord, Hittable},
};

pub struct Renderables {
    hittable_renderables: Vec<Arc<AnyHittable>>,
}

impl Renderables {
    pub fn new() -> Self {
        Self {
            hittable_renderables: Vec::new(),
        }
    }

    pub fn add_hittable<H>(&mut self, hittable: H)
    where
        H: Into<Arc<AnyHittable>>,
    {
        self.hittable_renderables.push(hittable.into());
    }
}

impl Hittable for Renderables {
    fn hit(&self, ray: &crate::ray::Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_interval.max();

        for hittable in self.hittable_renderables.iter() {
            let new_interval = Interval::new(ray_interval.min(), closest_so_far);
            if let Some(current_hit_record) = hittable.hit(ray, new_interval) {
                if hit_record.is_none()
                    || (hit_record.is_some()
                        && current_hit_record.t() < hit_record.as_ref().unwrap().t())
                {
                    closest_so_far = current_hit_record.t();
                    hit_record = Some(current_hit_record);
                }
            }
        }

        hit_record
    }
}
