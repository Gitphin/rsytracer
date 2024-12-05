use crate::vec3::Vec3;
 
#[allow(dead_code)]
pub type Color = Vec3;
 
#[allow(dead_code)]
pub fn write_color(pixel_color: Color) -> String {
    let r = (255.999 * pixel_color.x) as i32;
    let g = (255.999 * pixel_color.y) as i32;
    let b = (255.999 * pixel_color.z) as i32;
    let output: String = format!("{} {} {}\n", r, g, b);
    output
}
