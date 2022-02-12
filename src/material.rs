use crate::Vec3;

use crate::Rng;
use crate::random_unit_vector;
use crate::random_in_unit_sphere;

use crate::ray::*;

pub trait Material {
    fn scatter(&self, ray : &Ray, rec : &HitRecord, rng : &mut impl Rng) -> Ray;
}

pub struct Lambertian(Vec3);

impl Material for Lambertian {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        let n = hit.normal.into_inner();
        let bounce_dir = n + random_unit_vector(rng).into_inner();
        ray.attenuated(hit.point, bounce_dir, self.0)
    }
}

pub struct Metal {
    albedo : Vec3,
    fuzziness : f32,
}

fn reflect(v : Vec3, n : Vec3) -> Vec3 {
    return v - 2.0 * v.dot(&n) * n;
}

impl Material for Metal {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        let reflected = reflect(ray.direction.into_inner(), hit.normal.into_inner());
        let fuzzed = reflected + self.fuzziness * random_in_unit_sphere(rng);
        let keep = fuzzed.dot(&hit.normal) > 0.;
        let attenuation = if keep { self.albedo } else { Vec3::default() };
        ray.attenuated(hit.point, fuzzed, attenuation)
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
    pub fn metal(albedo : Vec3, fuzziness : f32) -> Self {
        Self::Metal(Metal { albedo, fuzziness })
    }
}

impl Material for SomeMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        match self {
            SomeMaterial::Lambertian(mat) => mat.scatter(ray, hit, rng),
            SomeMaterial::Metal(mat) => mat.scatter(ray, hit, rng),
        }
    }
}
