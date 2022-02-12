#![feature(test)]

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

extern crate nalgebra as na;
use na::{UnitVector3, Vector3, Point3};
type Vec3 = Vector3<f32>;
type UVec3 = UnitVector3<f32>;

extern crate rand;
use rand::distributions::Standard;
use rand::Rng;
extern crate rand_pcg;
use rand_pcg::Pcg32;

mod camera;
mod framebuf;
mod ray;
mod rgb;
mod scene;

use framebuf::Framebuf;
use ray::*;
use rgb::RGBu8;
use rgb::RGBf32;
use scene::*;
use camera::Camera;

impl From<Vec3> for RGBf32 {
    fn from(vec : Vec3) -> RGBf32 {
        RGBf32{ r : vec.x, g : vec.y, b : vec.z }
    }
}

fn rand_f32(rng : &mut impl Rng) -> f32 {
    rng.sample::<f32, Standard>(Standard)
}

fn random_in_unit_sphere(rng : &mut impl Rng) -> Vec3 {
    loop {
        let p = Vec3::new(rand_f32(rng), rand_f32(rng), rand_f32(rng));
        if p.norm() <= 1.0 { return p; }
    }
}

fn random_unit_vector(rng : &mut impl Rng) -> UVec3 {
    UVec3::new_normalize(random_in_unit_sphere(rng))
}

fn ray_color(world : &impl Shape, ray : &Ray, rng : &mut impl Rng, depth : i32) -> RGBf32 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    if depth == 0 {
        return RGBf32::black();
    }

    match world.hit(ray) {
        Some(hit) => {
            let n = hit.normal.into_inner();
            let bounce_dir = n + random_unit_vector(rng).into_inner();
            // return RGBf32::from(0.5 * (n + Vec3::repeat(1.0)));
            let new_ray = Ray::new_normalize(hit.point, bounce_dir);
            return ray_color(world, &new_ray, rng, depth - 1) * 0.5;
        }
        None => ()
    }

    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);
    RGBf32::from(white.lerp(&blue, t))
}

fn write_ppm<Pixel>(path : &Path, buf : &Framebuf<Pixel>)
  where RGBu8 : From<Pixel>, Pixel : Copy {
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(&mut file, "P3\n{} {}\n255\n", buf.width, buf.height).unwrap();
    for y in (0..buf.height).rev() {
        for x in 0..buf.width {
            let RGBu8{ r, g, b } = RGBu8::from(buf[(x, y)]);
            write!(&mut file, "{r} {g} {b}\n").unwrap();
        }
    }
}

fn main() {
    let width = 1280;
    let height = 800;
    let mut fbuf = Framebuf::new(width, height, RGBf32::default());

    let mut rng = Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);

    let camera = Camera::new(width, height);

    let mut world = Scene::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.));

    let samples = 100;
    let sample_weight = 1. / (samples as f32);
    let max_depth = 50;

    // Render
    for y in (0..height).rev() {
        for x in 0..width {
            let mut sum = RGBf32::default();
            for _ in 0..samples {
                let u = (x as f32) + rand_f32(&mut rng);
                let v = (y as f32) + rand_f32(&mut rng);
                let r = camera.cast(u, v);
                sum += ray_color(&world, &r, &mut rng, max_depth);
            }
            fbuf[(x, y)] = (sum * sample_weight).gamma_correct();
        }
    }

    write_ppm(Path::new("frame.ppm"), &fbuf);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;
    use crate::*;

    #[bench]
    fn sphere_ray_hit(b: &mut Bencher) {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = UVec3::new_normalize(
            Vec3::new(-0.016251257, 0.4996877, -0.86605316));
        let ray = Ray { origin, direction };

        b.iter(|| sphere.hit(&ray));
    }

    #[bench]
    fn bench_ray_color(b: &mut Bencher) {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = UVec3::new_normalize(
            Vec3::new(-0.016251257, 0.4996877, -0.86605316));
        let ray = Ray { origin, direction };

        b.iter(|| ray_color(&ray));
    }
}
