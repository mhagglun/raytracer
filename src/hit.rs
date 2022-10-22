use std::rc::Rc;

use crate::{
    material::Scatter,
    ray::Ray,
    vec3::{Point3D, Vec3},
};

pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vec3,
    pub outward_facing: bool,
    pub t: f32,
    pub mtrl: Rc<dyn Scatter>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct World {
    pub items: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.items.push(item);
    }

    // pub fn clear(&mut self) {
    //     while self.items.len() > 0 {
    //         self.items.pop();
    //     }
    // }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for item in &self.items {
            if let Some(rec) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}
