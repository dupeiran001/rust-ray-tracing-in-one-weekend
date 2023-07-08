use crate::color::*;
use crate::ray::*;
use crate::vec3::*;

mod color;
mod ray;
mod vec3;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16f64 / 9f64;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

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
            let pixel_color = ray_color(&r);

            write_color(std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone");
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::from(0f64, 0f64, -1f64), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::from(0.0, 0.0, -1.0)));
        return 0.5 * Color::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1f64);
    (1f64 - t) * Color::from(1f64, 1f64, 1f64) + t * Color::from(0.5, 0.7, 1f64)
}
