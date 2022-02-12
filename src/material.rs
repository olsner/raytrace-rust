use crate::Vec3;
use crate::UVec3;

use crate::Rng;
use crate::random_unit_vector;
use crate::random_in_unit_sphere;
use crate::rand_f32;

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

fn reflect(v : UVec3, n : UVec3) -> Vec3 {
    return v.into_inner() - 2.0 * v.dot(&n) * n.into_inner();
}

impl Material for Metal {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        let reflected = reflect(ray.direction, hit.normal);
        let fuzzed = reflected + self.fuzziness * random_in_unit_sphere(rng);
        let keep = fuzzed.dot(&hit.normal) > 0.;
        let attenuation = if keep { self.albedo } else { Vec3::default() };
        ray.attenuated(hit.point, fuzzed, attenuation)
    }
}

pub struct Dielectric {
    ir : f32
}

fn refract(uv : UVec3, n : UVec3, cos_theta : f32, ratio : f32) -> Vec3 {
    let r_out_perp = ratio * uv.into_inner() + (ratio * cos_theta) * n.into_inner();
    let r_out_para = -(1.0 - r_out_perp.norm_squared()).sqrt() * n.into_inner();
    return r_out_perp + r_out_para;
}

fn reflectance(cos_theta : f32, ratio : f32) -> f32 {
    let mut r0 = (1.0 - ratio) / (1.0 + ratio);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cos_theta).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        let ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };
        let cos_theta = (-hit.normal.dot(&ray.direction)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction =
            if ratio * sin_theta > 1.0 ||
                    reflectance(cos_theta, ratio) > rand_f32(rng) {
                reflect(ray.direction, hit.normal)
            } else {
                refract(ray.direction, hit.normal, cos_theta, ratio)
            };
        Ray::new_normalize(hit.point, direction, ray.attenuation)
    }
}

pub enum SomeMaterial {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl SomeMaterial {
    pub fn lambertian(albedo : Vec3) -> Self {
        Self::Lambertian(Lambertian(albedo))
    }
    pub fn metal(albedo : Vec3, fuzziness : f32) -> Self {
        Self::Metal(Metal { albedo, fuzziness })
    }
    pub fn dielectric(ir : f32) -> Self {
        Self::Dielectric(Dielectric{ ir })
    }
}

impl Material for SomeMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, rng : &mut impl Rng) -> Ray {
        match self {
            SomeMaterial::Lambertian(mat) => mat.scatter(ray, hit, rng),
            SomeMaterial::Metal(mat) => mat.scatter(ray, hit, rng),
            SomeMaterial::Dielectric(mat) => mat.scatter(ray, hit, rng),
        }
    }
}
