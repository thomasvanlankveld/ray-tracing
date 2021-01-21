use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        ray_in: Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
