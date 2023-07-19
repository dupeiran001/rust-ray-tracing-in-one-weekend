
# Antialiasing

When a real camera takes a picture, there are usually no jaggies along edges because the edge pixels are a blend of some foreground and some background. We can get the same effect by averaging a bunch of samples inside each pixel. We will not bother with stratification. This is controversial, but is usual for my programs. For some ray tracers it is critical, but the kind of general one we are writing doesn’t benefit very much from it and it makes the code uglier. We abstract the camera class a bit so we can make a cooler camera later.

## Some Random Number Utilities

One thing we need is a random number generator that returns real random numbers. We need a function that returns a canonical random number which by convention returns a random real in the range \\(0\leq r<1\\) . The “less than” before the 1 is important as we will sometimes take advantage of that.

A simple approach to this is to use the [`rand`](https://docs.rs/rand/latest/rand/) crate. We need to modify the `Cargo.toml` first:

```toml
[dependencies]
rand = "0.8.5"
```

For example, the *rand::random::<u8>()* function can generate a random `u8` in the range `u8::MIN` and `u8::MAX`. Hence we can get a real random number as desired with the following code snippet, added to `rtweekend.rs`:

```rust

#[inline]
pub fn random_double() -> f64 {
    rand::random::<u32>() as f64 / (std::u32::MAX as f64)
}

#[inline]
pub fn random_double_rng(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
```

## Generating Pixels with Multiple Samples

For a given pixel we have several samples within that pixel and send rays through each of the samples. The colors of these rays are then averaged:

![Pixel samples](../resources/pictures/fig-1.07-pixel-samples.jpg)

Now's a good time to create a `camera` class to manage our virtual camera and the related tasks of scene scampling. The following implements a simple camera using the axis-aligned camera from before:

```rust
use crate::{ray::Ray, vec3::*};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin_t = Point3::from(0f64, 0f64, 0f64);

        let horizonal_t = Vec3::from(viewport_width, 0.0, 0.0);
        let vertical_t = Vec3::from(0.0, viewport_height, 0.0);

        Camera {
            origin: origin_t,
            lower_left_corner: origin_t
                - horizonal_t / 2.0
                - vertical_t / 2.0
                - Vec3::from(0.0, 0.0, focal_length),
            vertical: vertical_t,
            horizonal: horizonal_t,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::from(
            self.origin,
            self.lower_left_corner + u * self.horizonal + v * self.vertical - self.origin,
        )
    }
}

```

To handle the multi-sampled color computation, we'll update the `write_color()` function. Rather than adding in a fractional contribution each time we acumulate more light to the color, just add the full color each iteration, and then perform a single divide at the end (by the number of samples) when writing out the color. In addition, we'll add a handy utility function to the `rtweekend.rs` utility header: `clamp(x,min,max)`, which clamps the value x to the `range [min,max]`:

```rust
#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

```

```rust
use crate::{rtweekend::clamp, vec3::*};

pub fn write_color<T: std::io::Write>(
    mut fmt: T,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let mut r = *pixel_color.x();
    let mut g = *pixel_color.y();
    let mut b = *pixel_color.z();

    // Divide the color by the number of samples.

    let scale = 1.0 / samples_per_pixel as f64;

    r *= scale;
    g *= scale;
    b *= scale;

    fmt.write_fmt(format_args!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    ))
}

```

main is also changed:

```rust

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16f64 / 9f64;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

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

                pixel_color += ray_color(&r, &world);
            }
            write_color(std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }
    eprintln!("\nDone");
}
```

Zooming into the image that is produced, we can see the difference in edge pixels.

![Before and after antialiasing](../resources/pictures/img-1.06-antialias-before-after.png)
