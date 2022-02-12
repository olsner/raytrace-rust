use crate::Point3;
use crate::UVec3;
use crate::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin : Point3<f32>,
    pub direction : UVec3,
    pub attenuation : Vec3,
}

impl Ray {
    pub fn new_normalize(origin : Point3<f32>, direction : Vec3,
            attenuation : Vec3) -> Ray {
        Ray { origin: origin, direction: UVec3::new_normalize(direction),
              attenuation: attenuation }
    }

    pub fn at(&self, t : f32) -> Point3<f32> {
        self.origin + (t * self.direction.into_inner())
    }

    pub fn attenuated(&self, new_origin : Point3<f32>, new_dir : Vec3,
            attenuation : Vec3) -> Ray {
        let attenuated = self.attenuation.component_mul(&attenuation);
        Ray::new_normalize(new_origin, new_dir, attenuated)
    }
}

pub struct HitRecord {
    pub point : Point3<f32>,
    pub normal : UVec3,
    pub distance : f32,
}

