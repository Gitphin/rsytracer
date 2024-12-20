use rayon::prelude::*;
use std::io::{self, Write};

mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;


fn main() {
    // OUR IMAGE DIMENSIONS
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_W: i32 = 400;
    const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // CREATE OUR WORLD + OBJECTS
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // ADD A CAMERA TO OUR SCENE
    let cam = Camera::new();

    // WHERE WE WILL STORE OUR COLOR COORDS
    let mut ppm_out = String::new();
    ppm_out.push_str(&format!("P3\n{} {}\n255\n", IMG_W, IMG_H));

    let img_w_minus_1 = (IMG_W - 1) as f64;
    let img_h_minus_1 = (IMG_H - 1) as f64;
    
    // PARALLELIZE INTO A VECTOR AND COLLECTS
    let rows: Vec<String> = (0..IMG_H)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut row = String::new();
            for i in 0..IMG_W {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_double()) / img_w_minus_1;
                    let v = (j as f64 + common::random_double()) / img_h_minus_1;
                    let r = cam.get_ray(u, v);
                    pixel_color += r.ray_color(&world);
                }
                row.push_str(&color::write_color(pixel_color, SAMPLES_PER_PIXEL));
            }
            row
        })
        .collect();

    ppm_out.extend(rows);

    if let Err(e) = io::stdout().write_all(ppm_out.as_bytes()) {
        eprintln!("Write to PPM failed with error: {}", e);
    } else {
        eprintln!("\nPPM Image Completed.");
    }
}

