use rayon::prelude::*;
use std::io::{self, Write};

mod vec3;

fn main() {
    // IMG DIMENSIONS
    const IMG_W: i16 = 256;
    const IMG_H: i16 = 256;

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
                let r = (i as f64) / img_w_minus_1;
                let g = (j as f64) / img_h_minus_1;
                let b = 0.25;

                // INTEGER CONVERSION
                let int_r = (255.999 * r) as i32;
                let int_g = (255.999 * g) as i32;
                let int_b = (255.999 * b) as i32;

                row.push_str(&format!("{} {} {}\n", int_r, int_g, int_b));
            }
            row
        })
        .collect();

    ppm_out.extend(rows);

    eprintln!("\nPPM Image Completed.");
    io::stdout().write_all(ppm_out.as_bytes()).unwrap_or(());
}

