use crate::Vec3;

use crate::Rng;
use crate::random_unit_vector;

use crate::ray::*;

pub trait Material {
    fn scatter(&self, ray : &Ray, rec : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3);
}

pub struct Lambertian(pub Vec3);

impl Material for Lambertian {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3) {
        let n = hit.normal.into_inner();
        let bounce_dir = n + random_unit_vector(rng).into_inner();
        (Ray::new_normalize(hit.point, bounce_dir), self.0)
    }
}

pub enum SomeMaterial {
    Lambertian(Lambertian),
}

impl SomeMaterial {
    pub fn lambertian(albedo : Vec3) -> Self {
        Self::Lambertian(Lambertian(albedo))
    }
}

impl Material for SomeMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3) {
        match self {
            SomeMaterial::Lambertian(mat) => mat.scatter(ray, hit, rng),
        }
    }
}
