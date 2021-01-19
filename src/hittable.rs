use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
