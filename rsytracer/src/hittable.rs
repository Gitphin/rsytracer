use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub front_facing: bool,
    pub t: f64,
}
 
#[allow(dead_code)]
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_facing = vec3::dot_product(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_facing {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
 
#[allow(dead_code)]
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
