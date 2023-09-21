use crate::{
    interval::Interval,
    objects::{HitRecord, Hittable},
};

pub struct Renderables {
    hittable_renderables: Vec<Box<dyn Hittable>>,
}

impl Renderables {
    pub fn new() -> Self {
        Self {
            hittable_renderables: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: impl Hittable + 'static) {
        self.hittable_renderables.push(Box::new(hittable));
    }
}

impl Hittable for Renderables {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_interval: Interval,
        hit_record: &mut crate::objects::HitRecord,
    ) -> bool {
        let mut temporary_hit_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_interval.max();

        for hittable in self.hittable_renderables.iter() {
            let new_interval = Interval::new(ray_interval.min(), closest_so_far);
            if hittable.hit(ray, new_interval, &mut temporary_hit_record) {
                hit_anything = true;
                closest_so_far = temporary_hit_record.t();
                hit_record.copy_from(&temporary_hit_record);
            }
        }

        hit_anything
    }
}
