use std::rc::Rc;

use hittable::HitRecord;
use hittable::Hittable;

use crate::color::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::sphere::*;
use crate::vec3::*;

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

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0f64, 0f64, -1f64), 0.5)));
    world.add(Rc::new(Sphere::from(
        Point3::from(0f64, -100.5f64, -1f64),
        100f64,
    )));

    // Camera

    let viewport_height = 2f64;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1f64;

    let origin = Point3::from(0f64, 0f64, 0f64);
    let horizontal = Vec3::from(viewport_width, 0f64, 0f64);
    let vertical = Vec3::from(0f64, viewport_height, 0f64);
    let lower_left_corner =
        origin - horizontal / 2f64 - vertical / 2f64 - Vec3::from(0f64, 0f64, focal_length);

    // Render

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH as f64 - 1f64);
            let v = (j as f64) / (IMAGE_HEIGHT as f64 - 1f64);

            let r: Ray = Ray::from(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);

            write_color(std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone");
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0f64, Infinity) {
        return 0.5 * (rec.normal() + Color::from(1f64, 1f64, 1f64));
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1f64);
    (1f64 - t) * Color::from(1f64, 1f64, 1f64) + t * Color::from(0.5, 0.7, 1f64)
}
