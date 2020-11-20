use crate::core::color::Color;
use crate::core::geometry::{Point3, Vec3, Vector};
use crate::core::math::rand::{rand, rand_between};
use crate::core::time::Interval;
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::scene::object::movingsphere::MovingSphere;
use crate::scene::object::sphere::Sphere;
use crate::scene::{Hit, HitList};
use crate::textures::Checker;
use std::rc::Rc;

pub fn generate_random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material = Rc::new(Lambertian::new(Rc::new(Checker::from_color(
        Color(0.2, 0.3, 0.1),
        Color(0.9, 0.9, 0.9),
    ))));

    world.add(Box::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand();

            let center = {
                let (rand_1, rand_2) = (rand(), rand());

                Point3(
                    rand_1.mul_add(0.9, a as f64),
                    0.2,
                    rand_2.mul_add(0.9, b as f64),
                )
            };

            let object: Box<dyn Hit>;

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::rand() * Color::rand();

                    sphere_material = Rc::new(Lambertian::from_color(albedo));

                    let center2 = center + Vec3(0.0, rand_between(0.0, 0.5), 0.0);
                    object = Box::new(MovingSphere::new(
                        center,
                        center2,
                        Interval::new(0.0, 1.0),
                        0.2,
                        sphere_material,
                    ));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::rand_between(0.5, 1.0);
                    let fuzz = rand_between(0.0, 0.5);

                    sphere_material = Rc::new(Metal::new(albedo, fuzz));

                    object = Box::new(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // Glass
                    sphere_material = Rc::new(Dielectric::new(1.5));

                    object = Box::new(Sphere::new(center, 0.2, sphere_material));
                }

                world.add(object);
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::from_color(Color(0.4, 0.4, 0.1)));
    let material3 = Rc::new(Metal::new(Color(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(Point3(0.0, 1.0, 0.0), 1.0, material1)));

    world.add(Box::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    world.add(Box::new(Sphere::new(Point3(4.0, 1.0, 0.0), 1.0, material3)));
    world
}

#[allow(dead_code)]
pub fn generate_scene_two_spheres() -> HitList {
    let checker_texture = Rc::new(Checker::from_color(
        Color(0.2, 0.3, 0.1),
        Color(0.9, 0.9, 0.9),
    ));

    let mut hitlist = HitList::new();
    let lambertian = Rc::new(Lambertian::new(checker_texture));

    hitlist.add(Box::new(Sphere::new(
        Point3(0.0, -10.0, 0.0),
        1.0,
        lambertian.clone(),
    )));
    hitlist.add(Box::new(Sphere::new(
        Point3(0.0, 10.0, 0.0),
        1.0,
        lambertian,
    )));

    hitlist
}
