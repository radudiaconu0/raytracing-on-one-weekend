use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}
impl HittableList {
    pub fn new(&mut self, object: Rc<dyn Hittable>) {
        self.add(object)
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = HitRecord::default();

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_t(temp_rec.t());
                rec.set_p(temp_rec.p());
                rec.set_normal(temp_rec.normal());
                rec.set_mat(Rc::clone(&temp_rec.mat));
                rec.set_front_face(temp_rec.front_face);
            }
        }
        hit_anything
    }
}
