use std::rc::Rc;

use hittable::HitRecord;
use hittable::Hittable;

use crate::camera::Camera;
use crate::color::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::sphere::*;
use crate::vec3::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16f64 / 9f64;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0f64, 0f64, -1f64), 0.5)));
    world.add(Rc::new(Sphere::from(
        Point3::from(0f64, -100.5f64, -1f64),
        100f64,
    )));

    // Camera

    let cam = Camera::new();

    // Render

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::from(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }
    eprintln!("\nDone");
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::from(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.0001f64, Infinity) {
        let target: Point3 = rec.p() + Vec3::random_in_hemisphere(&rec.normal());
        return 0.5 * ray_color(&Ray::from(rec.p(), target - rec.p()), world, depth - 1);
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1f64);
    (1f64 - t) * Color::from(1f64, 1f64, 1f64) + t * Color::from(0.5, 0.7, 1f64)
}
