use rand::Rng;
 
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
 
/// COMMONLY USED FUNCTIONS WHICH ARE BASIC
/// ENOUGH TO THE POINT I DONT NEED TO GET
/// A CRATE FOR THEM

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}
 
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
