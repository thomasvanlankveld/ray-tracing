use crate::{color::Color, hittable::HitRecord, material::Material, point3::Vec3, ray::Ray};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if record.is_front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(ray_in.direction);
        let refracted = Vec3::refract(unit_direction, record.normal, refraction_ratio);

        *scattered = Ray::new(record.point, refracted);
        return true;
    }
}
