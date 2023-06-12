use crate::color::*;
use crate::vec3::*;

mod color;
mod vec3;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Color::from(
                (i as f64) / (IMAGE_WIDTH as f64 - 1f64),
                (j as f64) / (IMAGE_HEIGHT as f64 - 1f64),
                0.25,
            );

            write_color(std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone");
}
