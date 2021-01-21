use crate::{color::Color, material::Material, ray::Ray};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray,
        record: &crate::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let reflected = ray_in.direction.unit_vector().reflect(record.normal);
        *scattered = Ray::new(record.point, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(record.normal) > 0.
    }
}
