use crate::ray::*;
use crate::Point3;
use crate::UVec3;

#[derive(Clone)]
#[derive(Copy)]
pub struct Sphere {
    center : Point3<f32>,
    radius : f32,
}

impl Sphere {
    pub fn new(center : Point3<f32>, radius : f32) -> Sphere {
        Sphere{ center, radius }
    }
}

pub trait Shape {
    fn hit(self : &Self, ray : &Ray) -> Option<HitRecord>;
}

impl Shape for Sphere {
    fn hit(self : &Sphere, r : &Ray) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let hb = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = hb * hb - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let distance = (-hb - discriminant.sqrt()) / a;
        let point = r.at(distance);
        let normal = UVec3::new_normalize(point - self.center);
        if distance >= 0.0 {
            Some(HitRecord{ point, normal, distance })
        }
        else {
            None
        }
    }
}

fn best(left : Option<HitRecord>, right : Option<HitRecord>) -> Option<HitRecord> {
    match left {
        None => right,
        Some(lhit) => Some(match right {
            Some(rhit) =>
                if lhit.distance < rhit.distance { lhit } else { rhit }
            None => lhit,
        })
    }
}

pub struct Scene {
    spheres : Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { spheres : Vec::new() }
    }

    pub fn add(self : &mut Self, sphere : Sphere) {
        self.spheres.push(sphere);
    }
}

impl Shape for Scene {
    fn hit(self : &Scene, r : &Ray) -> Option<HitRecord> {
        let mut best_hit = None;
        for sphere in &self.spheres {
            best_hit = best(sphere.hit(r), best_hit);
        }
        return best_hit;
    }
}

