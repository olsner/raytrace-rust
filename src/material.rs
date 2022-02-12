use crate::Vec3;

use crate::Rng;
use crate::random_unit_vector;

use crate::ray::*;

pub trait Material {
    fn scatter(&self, ray : &Ray, rec : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3);
}

pub struct Lambertian(pub Vec3);

impl Material for Lambertian {
    fn scatter(&self, _ : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3) {
        let n = hit.normal.into_inner();
        let bounce_dir = n + random_unit_vector(rng).into_inner();
        (Ray::new_normalize(hit.point, bounce_dir), self.0)
    }
}

pub struct Metal(pub Vec3);

fn reflect(v : Vec3, n : Vec3) -> Vec3 {
    return v - 2.0 * v.dot(&n) * n;
}

impl Material for Metal {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, _ : &mut impl Rng) -> (Ray, Vec3) {
        let reflected = reflect(ray.direction.into_inner(), hit.normal.into_inner());
        let new_ray = Ray::new_normalize(hit.point, reflected);
        if reflected.dot(&hit.normal) > 0. {
            (new_ray, self.0)
        } else {
            (new_ray, Vec3::default())
        }
    }
}

pub enum SomeMaterial {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl SomeMaterial {
    pub fn lambertian(albedo : Vec3) -> Self {
        Self::Lambertian(Lambertian(albedo))
    }
    pub fn metal(albedo : Vec3) -> Self {
        Self::Metal(Metal(albedo))
    }
}

impl Material for SomeMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> (Ray, Vec3) {
        match self {
            SomeMaterial::Lambertian(mat) => mat.scatter(ray, hit, rng),
            SomeMaterial::Metal(mat) => mat.scatter(ray, hit, rng),
        }
    }
}
