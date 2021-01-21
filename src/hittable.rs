use std::rc::Rc;

use crate::{
    color::Color, lambertian::Lambertian, material::Material, point3::Point3, ray::Ray, vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            material: Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
            is_front_face: false,
        }
    }

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
