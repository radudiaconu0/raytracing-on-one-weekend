use std::rc::Rc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
    pub fn p(&self) -> Point3 {
        self.p
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn set_mat(&mut self, mat: Rc<Material>) {
        self.mat = mat;
    }
    pub fn set_front_face(&mut self, front_face: bool) {
        self.front_face = front_face;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
