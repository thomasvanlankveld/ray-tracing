use rand::random;

use crate::{color::Color, hittable::HitRecord, material::Material, point3::Vec3, ray::Ray};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;
    return r0 + (1. - r0) * f64::powi(1. - cosine, 5);
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
        let cos_theta = f64::min((-unit_direction).dot(record.normal), 1.);
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

        let can_refract = refraction_ratio * sin_theta <= 1.;
        let direction =
            if can_refract && reflectance(cos_theta, refraction_ratio) <= random::<f64>() {
                unit_direction.refract(record.normal, refraction_ratio)
            } else {
                unit_direction.reflect(record.normal)
            };

        *scattered = Ray::new(record.point, direction);

        true
    }
}
