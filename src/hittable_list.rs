#[allow(unused_imports)]
use crate::hittable::{self, HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec = record.clone();
        let mut was_anything_hit = false;
        let mut closest_so_far = t_max.clone();

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                was_anything_hit = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }

        was_anything_hit
    }
}
