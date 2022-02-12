use crate::ray::*;
use crate::Point3;
use crate::UVec3;
use crate::Vec3;

use crate::Rng;
use crate::random_unit_vector;

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
    fn hit(&self, ray : &Ray) -> Option<HitRecord>;
}

impl Shape for Sphere {
    fn hit(&self, r : &Ray) -> Option<HitRecord> {
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

fn best(i : usize, left : Option<HitRecord>, right : Option<(usize, HitRecord)>) -> Option<(usize, HitRecord)> {
    match left {
        None => right,
        Some(lhit) => Some(match right {
            Some(rhit) =>
                if lhit.distance < rhit.1.distance { (i, lhit) } else { rhit }
            None => (i, lhit),
        })
    }
}

fn sky_color(dir : UVec3) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let t = 0.5 * (dir.y + 1.0);
    white.lerp(&blue, t)
}

pub struct Scene {
    spheres : Vec<Sphere>,
    materials: Vec<SomeMaterial>
}

impl Scene {
    pub fn new() -> Self {
        Scene { spheres : Vec::new(), materials : Vec::new() }
    }

    pub fn add(&mut self, sphere : Sphere, material : SomeMaterial) {
        self.spheres.push(sphere);
        self.materials.push(material);
    }

    fn hit(&self, r : &Ray) -> Option<(usize, HitRecord)> {
        let mut best_hit = None;
        for i in 0..self.spheres.len() {
            let sphere = &self.spheres[i];
            best_hit = best(i, sphere.hit(r), best_hit);
        }
        return best_hit;
    }

    pub fn ray_color(&self, ray : &Ray, rng : &mut impl Rng, depth : i32) -> Vec3 {
        if depth == 0 {
            return Vec3::repeat(0.0);
        }

        match self.hit(ray) {
            Some((index, hit)) => {
                let mat = &self.materials[index];
                let (new_ray, attenuation) = mat.scatter(ray, &hit, rng);
                self.ray_color(&new_ray, rng, depth - 1).component_mul(&attenuation)
            }
            None => sky_color(ray.direction)
        }
    }
}

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
