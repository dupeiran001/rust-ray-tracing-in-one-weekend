use crate::vec3::*;

pub fn write_color<T: std::io::Write>(mut fmt: T, pixel_color: Color) -> std::io::Result<()> {
    fmt.write_fmt(format_args!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    ))
}
