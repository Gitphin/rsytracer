use crate::vec3::Vec3;
use crate::common::{self, clamp};
 
#[allow(dead_code)]
pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
 
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // MAY NEED TO CHANGE THIS BACK TO I32
    let a = (256.0 * clamp(r, 0.0, 0.999)) as f64;
    let b = (256.0 * clamp(g, 0.0, 0.999)) as f64;
    let c = (256.0 * clamp(b, 0.0, 0.999)) as f64;
 

    let output: String = format!("{} {} {}\n", a, b, c);
    output
}
