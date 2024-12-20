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
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;
use vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Result<Color, ()> {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return Ok(0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0)));
    } else {
        Err(())
    }
}

fn main() {
    // IMG DIMENSIONS
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_W: i32 = 400;
    const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_double()) / (img_w_minus_1) as f64;
                    let v = (j as f64 + common::random_double()) / (img_h_minus_1) as f64;
                    let r = cam.get_ray(u, v);
                    let v = ray_color(&r, &world);
                    match v {
                        Ok(v) => pixel_color += v,
                        Err(_) => ()
                    }
                }
                let output = color::write_color(pixel_color, SAMPLES_PER_PIXEL);
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
