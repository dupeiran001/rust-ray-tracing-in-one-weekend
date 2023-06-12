
# The vec3 struct

Every one having used Cuda or some framework should be familiar with vec3 or .etc.
Almost every graphic program have some structs for storing geometric vector or colors.
In many systems these vectors are 4D (3D plus a homogeneous coordinate for geometry,
and RGB plus an alpha transparency channel for colors). In our usage, three coordinates
suffices. We'll use the same struct `vec3` for colors, locations, directions, offsets, whatever.
Some people don’t like this because it doesn’t prevent you from doing something silly, like adding
color to a location. They have a good point, but we’re going to always take the “less code” route when
not obviously wrong. In spite of this, we do declare two aliases for `vec3`: `point3` and `color`.
We use them only to clarify intent and use.

## Variables and Methods

We need a new file for our `vec3` struct. To do this, create a new file in the same folder as `main.rs`,
naming `vec3.rs`, and add

```rust
mod vec3;
```

to the top of the `main.rs`.

Here is my implementation of `Vec3` struct

```rust
use core::ops::*;

#[derive(Debug)]
pub struct Vec3 {
    e: (f64, f64, f64),
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            e: (0f64, 0f64, 0f64),
        }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> &f64 {
        &self.e.0
    }

    pub fn y(&self) -> &f64 {
        &self.e.1
    }

    pub fn z(&self) -> &f64 {
        &self.e.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}
```

It's safe and convenient to implement clone and copy for our `Vec3` struct,
as it can be cloned by only clone the memory, it's completely safe:

```rust
impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self {
            e: (*self.x(), *self.y(), *self.z()),
        }
    }
}

impl Copy for Vec3 {}
```

And then, implement some operator trait for it

```rust

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            e: (-self.x(), -self.y(), -self.z()),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e.0 += rhs.x();
        self.e.1 += rhs.y();
        self.e.2 += rhs.z();
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e.0 *= rhs;
        self.e.1 *= rhs;
        self.e.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e.0 *= 1f64 / rhs;
        self.e.1 *= 1f64 / rhs;
        self.e.2 *= 1f64 / rhs;
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        assert!(index <= 2 && index >= 0);
        match index {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => unreachable!(),
        }
    }
}
```

as well as some tests:

```rust
#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn basic_ops() {
        let vec1 = Vec3::from(1f64, 2f64, 3f64);
        assert_eq!(1f64, *vec1.x());
        assert_eq!(2f64, *vec1.y());
        assert_eq!(3f64, *vec1.z());

        assert_eq!(1f64, vec1[0]);
        assert_eq!(2f64, vec1[1]);
        assert_eq!(3f64, vec1[2]);

        let vec2 = -vec1;

        assert_eq!(-1f64, *vec2.x());
        assert_eq!(-2f64, *vec2.y());
        assert_eq!(-3f64, *vec2.z());

        let mut vec3 = Vec3::from(5f64, 3f64, 1f64);
        vec3 += vec2;

        assert_eq!(4f64, *vec3.x());
        assert_eq!(1f64, *vec3.y());
        assert_eq!(-2f64, *vec3.z());

        vec3 *= 2f64;
        assert_eq!(8f64, *vec3.x());
        assert_eq!(2f64, *vec3.y());
        assert_eq!(-4f64, *vec3.z());

        assert_eq!(9f64, Vec3::from(1f64, 2f64, 2f64).length_squared());
        assert_eq!(3f64, Vec3::from(1f64, 2f64, 2f64).length());

        vec3 /= 2f64;
        assert_eq!(4f64, *vec3.x());
        assert_eq!(1f64, *vec3.y());
        assert_eq!(-2f64, *vec3.z());
    }
}

```

We use `f64` here, but some ray tracer use `f32`, that is float. Either is fine.

## `Vec3` utility Functions

The second part of the file contains vector utility functions.

```rust
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} {} {}", self[0], self[1], self[2]).as_str())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: (self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()),
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: (self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()),
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: (self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: (self.x() * rhs, self.y() * rhs, self.z() * rhs),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1f64 / rhs) * self
    }
}

impl Vec3 {
    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Self {
            e: (
                lhs.y() * rhs.z() - lhs.z() * rhs.y(),
                lhs.z() * rhs.x() - lhs.x() * rhs.z(),
                lhs.x() * rhs.y() - lhs.y() * rhs.x(),
            ),
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

```

And it's test:

```rust

    #[test]
    fn utility_funtions() {
        let vec1 = Vec3::from(1f64, 2f64, 3f64);

        assert_eq!(String::from("1 2 3"), vec1.to_string());

        let vec2 = Vec3::from(2f64, 3f64, 4f64);
        let vec3 = vec1 + vec2;
        assert_eq!(3f64, *vec3.x());
        assert_eq!(5f64, *vec3.y());
        assert_eq!(7f64, *vec3.z());

        let vec4 = vec3 - vec2;
        assert_eq!(vec4.x(), vec1.x());
        assert_eq!(vec4.y(), vec1.y());
        assert_eq!(vec4.z(), vec1.z());

        let vec5 = vec4 * vec2;
        assert_eq!(2f64, *vec5.x());
        assert_eq!(6f64, *vec5.y());
        assert_eq!(12f64, *vec5.z());

        let vec6 = vec4 * 2f64;
        assert_eq!(2f64, *vec6.x());
        assert_eq!(4f64, *vec6.y());
        assert_eq!(6f64, *vec6.z());

        let vec6 = 2f64 * vec4;
        assert_eq!(2f64, *vec6.x());
        assert_eq!(4f64, *vec6.y());
        assert_eq!(6f64, *vec6.z());

        let vec7 = vec6 / 2f64;
        assert_eq!(1f64, *vec7.x());
        assert_eq!(2f64, *vec7.y());
        assert_eq!(3f64, *vec7.z());

        assert_eq!(14f64, Vec3::dot(&vec7, &vec1));
        assert_eq!(Vec3::from(1f64, -2f64, 1f64), Vec3::cross(&vec3, &vec1));
        assert_eq!(
            Vec3::from(1f64 / 3f64, 2f64 / 3f64, 2f64 / 3f64),
            Vec3::from(1f64, 2f64, 2f64).unit_vector()
        )
    }
```

## Color Utility Functions

Using our `Vec3` struct, we'll create a utility function to write a
single pixel's color out to the standard output stream.

Write the following code to a new file called color.rs, add `mod color;` to
the top of main again.

```rust
use crate::vec3::*;

pub fn write_color<T: std::io::Write>(mut fmt: T, pixel_color: Color) -> std::io::Result<()> {
    fmt.write_fmt(format_args!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    ))
}


```

Now we can change our main to use this:

```rust
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

```

This should be able to generate the same output as the previous Chapter.
