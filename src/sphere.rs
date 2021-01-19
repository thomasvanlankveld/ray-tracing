use crate::hittable::{HitRecord, Hittable};
use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Point3, radius: f64) -> Self {
        return Self { center, radius };
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        };
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            };
        };

        record.t = root;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / self.radius;

        return true;
    }
}
