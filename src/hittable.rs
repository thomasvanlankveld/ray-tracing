use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.is_front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
