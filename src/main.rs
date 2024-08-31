use camera::Camera;
use color::Color;
use material::Material;
use std::rc::Rc;
use vec3::Vec3;

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();
    let material_ground = Rc::new(Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        &material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    Rc::new(Material::Metal { albedo, fuzz })
                } else {
                    // glass
                    Rc::new(Material::Dielectric { ir: 1.5 })
                };
                world.add(Rc::new(Sphere::new(center, 0.2, &material)));
            }
        }
    }
    let material = Rc::new(Material::Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        &material,
    )));
    let material = Rc::new(Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &material,
    )));

    let material = Rc::new(Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        &material,
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world);
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::vec3::Vec3;

    #[test]
    fn test_interval() {
        let i = Interval::new(0.0, 1.0);
        assert_eq!(i.clamp(0.5), 0.5);
        assert_eq!(i.clamp(1.5), 1.0);
        assert_eq!(i.clamp(-0.5), 0.0);
    }

    #[test]
    fn test_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }
}
