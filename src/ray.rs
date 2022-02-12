use crate::Point3;
use crate::UVec3;
use crate::Vec3;

pub struct Ray {
    pub origin : Point3<f32>,
    pub direction : UVec3,
}

impl Ray {
    pub fn new_normalize(origin : Point3<f32>, direction : Vec3) -> Ray {
        Ray { origin: origin, direction: UVec3::new_normalize(direction) }
    }

    pub fn at(self : &Self, t : f32) -> Point3<f32> {
        self.origin + (t * self.direction.into_inner())
    }
}

pub struct HitRecord {
    pub point : Point3<f32>,
    pub normal : UVec3,
    pub distance : f32,
}

