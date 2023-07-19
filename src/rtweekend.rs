pub use std::f64::consts::PI;
pub use std::f64::MAX as Infinity;

// Utility Functions
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    rand::random::<u32>() as f64 / (std::u32::MAX as f64)
}

#[inline]
pub fn random_double_rng(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

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
