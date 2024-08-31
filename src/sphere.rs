use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Material>,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: &Rc<Material>) -> Self {
        Self {
            center,
            radius,
            mat: mat.clone(),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        let outward_normal = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.set_mat(Rc::clone(&self.mat));
        true
    }
}
