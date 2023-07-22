use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point3::new(),
            normal: Vec3::new(),
            t: Default::default(),
            front_face: Default::default(),
        }
    }

    pub fn from(point: Point3, n: Vec3, hit_t: f64) -> Self {
        HitRecord {
            p: point,
            normal: n,
            t: hit_t,
            front_face: Default::default(),
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
