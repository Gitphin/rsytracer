use crate::common;
use crate::vec3::{Point3, Vec3};
use crate::Color;
use crate::HitRecord;
use crate::Hittable;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    /// Checks if our raycast detects a hit on a hittable object
    ///
    /// r - a Ray object, should check each texture coord of our img
    /// world - contains all of our Hittable trait objects in the scene
    pub fn ray_color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(self, 0.001, common::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        } else {
            let unit_direction = self.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
