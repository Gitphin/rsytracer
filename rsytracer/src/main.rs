use rayon::prelude::*;
use std::io::{self, Write};

mod vec3;
mod color;
mod ray;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
 
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let dir = r.direction();
    let a = vec3::dot_product(dir, dir);
    let b = 2.0 * vec3::dot_product(oc, dir);
    let c = vec3::dot_product(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    // IMG DIMENSIONS
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_W: i32 = 400;
    const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lleft_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut ppm_out = String::new();
    ppm_out.push_str(&format!("P3\n{} {}\n255\n", IMG_W, IMG_H));

    // MINIMIZE TYPE CONVERSION
    let img_w_minus_1 = (IMG_W - 1) as f64;
    let img_h_minus_1 = (IMG_H - 1) as f64;

    // PARALLELIZE OUR ITERATION, MAP J TO OUR ROWS
    let rows: Vec<String> = (0..IMG_H)
        .into_par_iter() 
        .rev()
        .map(|j| {
            let mut row = String::new();
            for i in 0..IMG_W {
            let u = i as f64 / img_w_minus_1;
            let v = j as f64 / img_h_minus_1;
            let r = Ray::new(
                origin,
                lleft_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            let output = color::write_color(pixel_color);
                row.push_str(&output);
            }
            row
        })
        .collect();

    ppm_out.extend(rows);
    // MAY NEED TO CHANGE HOW AM UNWRAPPING (RIGHT NOW NOT HANDLING BAD RESULT)
    io::stdout().write_all(ppm_out.as_bytes()).unwrap_or(());

    eprintln!("\nPPM Image Completed.");
}

