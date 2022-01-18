use crate::core::color::Color;
use crate::core::geometry::{Point3, Vec3, Vector};
use crate::core::math::rand::{rand, rand_between};
use crate::core::time::Interval;
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::scene::object::movingsphere::MovingSphere;
use crate::scene::object::sphere::Sphere;
use crate::scene::{Hit, HitList};
use crate::textures::Checker;
use std::sync::Arc;

fn generate_random_sphere(base_x: i32, base_y: i32) -> Option<Arc<dyn Hit>> {
    let choose_material = rand();

    let center = Point3(
        rand().mul_add(0.9, base_x as f64),
        0.2,
        rand().mul_add(0.9, base_y as f64),
    );

    if (center - Point3(4.0, 0.2, 0.0)).length() <= 0.9 {
        return None;
    }

    let object: Arc<dyn Hit>;
    let sphere_material: Arc<dyn Material>;

    if choose_material < 0.8 {
        // Diffuse
        let albedo = Color::rand() * Color::rand();

        sphere_material = Arc::new(Lambertian::from_color(albedo));

        let center2 = center + Vec3(0.0, rand_between(0.0, 0.5), 0.0);
        object = Arc::new(MovingSphere::new(
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

        sphere_material = Arc::new(Metal::new(albedo, fuzz));

        object = Arc::new(Sphere::new(center, 0.2, sphere_material));
    } else {
        // Glass
        sphere_material = Arc::new(Dielectric::new(1.5));

        object = Arc::new(Sphere::new(center, 0.2, sphere_material));
    }

    Some(object)
}

pub fn generate_random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material = Arc::new(Lambertian::new(Arc::new(Checker::from_color(
        Color(0.2, 0.3, 0.1),
        Color(0.9, 0.9, 0.9),
    ))));

    world.add(Arc::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for x in -11..11 {
        for y in -11..11 {
            if let Some(object) = generate_random_sphere(x, y) {
                world.add(object);
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::from_color(Color(0.4, 0.4, 0.1)));
    let material3 = Arc::new(Metal::new(Color(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(Point3(0.0, 1.0, 0.0), 1.0, material1)));

    world.add(Arc::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    world.add(Arc::new(Sphere::new(Point3(4.0, 1.0, 0.0), 1.0, material3)));
    world
}

#[allow(dead_code)]
pub fn generate_scene_two_spheres() -> HitList {
    let checker_texture = Arc::new(Checker::from_color(
        Color(0.2, 0.3, 0.1),
        Color(0.9, 0.9, 0.9),
    ));

    let mut hitlist = HitList::new();
    let lambertian = Arc::new(Lambertian::new(checker_texture));

    hitlist.add(Arc::new(Sphere::new(
        Point3(0.0, -10.0, 0.0),
        1.0,
        lambertian.clone(),
    )));
    hitlist.add(Arc::new(Sphere::new(
        Point3(0.0, 10.0, 0.0),
        1.0,
        lambertian,
    )));

    hitlist
}
