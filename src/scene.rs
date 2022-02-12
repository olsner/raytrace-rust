use crate::ray::*;
use crate::Point3;
use crate::UVec3;
use crate::Vec3;

use crate::Rng;
use crate::rand_f32;

use crate::material::*;

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
        let outward_normal = UVec3::new_normalize(point - self.center);
        let front_face = outward_normal.dot(&r.direction) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };
        if distance >= 0.0 {
            Some(HitRecord{ point, normal, distance, front_face })
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

fn sky_color(ray : &Ray) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let t = 0.5 * (ray.direction.y + 1.0);
    white.lerp(&blue, t).component_mul(&ray.attenuation)
}

pub struct Scene {
    spheres : Vec<Sphere>,
    materials: Vec<SomeMaterial>
}

fn very_small(vec : Vec3) -> bool {
    vec.x.abs() < 1e-7 && vec.y.abs() < 1e7 && vec.z.abs() < 1e7
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

    pub fn ray_color(&self, ray : &Ray, rng : &mut impl Rng, depth : u32) -> Vec3 {
        if depth == 0 /*|| very_small(ray.attenuation)*/ {
            return Vec3::repeat(0.0);
        }

        match self.hit(ray) {
            Some((index, hit)) => {
                let mat = &self.materials[index];
                let new_ray = mat.scatter(ray, &hit, rng);
                self.ray_color(&new_ray, rng, depth - 1)
            }
            None => sky_color(ray)
        }
    }
}

fn rand_color(rng : &mut impl Rng) -> Vec3 {
    Vec3::new(rand_f32(rng), rand_f32(rng), rand_f32(rng))
}

pub fn random_scene(rng : &mut impl Rng) -> Scene {
    let mut world = Scene::new();

    let mat_ground = SomeMaterial::lambertian(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0), mat_ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f32(rng);
            let center = Point3::new(a as f32 + 0.9 * rand_f32(rng),
                                     0.2,
                                     b as f32 + 0.9 * rand_f32(rng));
            let mid = Point3::new(4.0, 0.2, 0.0);

            if (center - mid).norm() > 0.9 {
                let mat = if choose_mat < 0.8 {
                    let albedo = rand_color(rng).component_mul(&rand_color(rng));
                    SomeMaterial::lambertian(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::repeat(0.5) + 0.5 * rand_color(rng);
                    let fuzz = rand_f32(rng) * 0.5;
                    SomeMaterial::metal(albedo, fuzz)
                } else {
                    SomeMaterial::dielectric(1.5)
                };

                world.add(Sphere::new(center, 0.2), mat);
            }
        }
    }

    let material1 = SomeMaterial::dielectric(1.5);
    let material2 = SomeMaterial::lambertian(Vec3::new(0.4, 0.2, 0.1));
    let material3 = SomeMaterial::metal(Vec3::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0), material1);
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0), material2);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0), material3);

    return world;
}
