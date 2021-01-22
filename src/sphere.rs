use std::rc::Rc;

use material::Material;

use crate::{
    hittable::{HitRecord, Hittable},
    material,
    point3::Point3,
    ray::Ray,
};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material,
        };
    }
}

impl Hittable for Sphere {
    // Whether a ray passes through this sphere
    //
    // If it does not hit, it returns -1. If it does, it returns distance t that must be travelled along the given ray to reach the sphere.
    //
    // A sphere at origin 0, 0, 0 is the collection of points where x^2+y^2+z^2=R^2
    //
    // Expressed in vector form where P is a point and C is the center of the sphere: (P−C)⋅(P−C)=r^2
    //
    // Using a ray P(t)=A+tb instead of a point, where A is its origin, b its direction and t the
    // distance along that direction, the expression becomes (P(t)−C)⋅(P(t)−C)=r^2
    //
    // Expanding this equation and moving everything to the left yields: (t^2)b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
    //
    // The function below checks whether t > 0 in (t^2)b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
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
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Rc::clone(&self.material);

        return true;
    }
}
