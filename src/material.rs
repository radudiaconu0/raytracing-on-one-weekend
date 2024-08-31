use crate::rtweekend::random_double;
use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 },
}

pub fn scatter(
    material: &Material,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = rec.normal();
            }

            *scattered = Ray::new(rec.p(), scatter_direction);
            *attenuation = *albedo;
            true
        }
        Material::Metal { albedo, fuzz } => {
            let fuzz = if *fuzz < 1.0 { *fuzz } else { 1.0 };
            let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal());
            *scattered = Ray::new(rec.p(), reflected + Vec3::random_in_unit_sphere() * fuzz);
            *attenuation = *albedo;
            Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0
        }
        Material::Dielectric { ir } => {
            *attenuation = Color::new(1.0, 1.0, 1.0);
            let refraction_ratio = if rec.front_face { 1.0 / *ir } else { *ir };

            let unit_direction = Vec3::unit_vector(r_in.direction());
            let cos_theta = Vec3::dot(&-unit_direction, &rec.normal()).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction =
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                    Vec3::reflect(&unit_direction, &rec.normal())
                } else {
                    Vec3::refract(&unit_direction, &rec.normal(), refraction_ratio)
                };

            *scattered = Ray::new(rec.p(), direction);
            true
        }
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
    }
}
