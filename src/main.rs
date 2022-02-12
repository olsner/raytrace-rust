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
mod material;
mod ray;
mod rgb;
mod scene;

use framebuf::Framebuf;
use material::*;
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
        let p1 = Vec3::new(rand_f32(rng), rand_f32(rng), rand_f32(rng));
        let p = 2.0 * p1 - Vec3::repeat(1.0);
        if p.norm_squared() <= 1.0 { return p; }
    }
}

fn random_unit_vector(rng : &mut impl Rng) -> UVec3 {
    UVec3::new_normalize(random_in_unit_sphere(rng))
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

    //let camera = Camera::new(width, height);
    let camera = Camera::look_from_at(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        UVec3::new_unchecked(Vec3::new(0.0, 1.0, 0.0)),
        40.0, // fov, 90 = zoomed out
        width, height);

    let mat_ground = SomeMaterial::lambertian(Vec3::new(0.8, 0.8, 0.0));
    let mat_center = SomeMaterial::lambertian(Vec3::new(0.1, 0.2, 0.5));
    let mat_left = SomeMaterial::dielectric(1.5);
    let mat_right = SomeMaterial::metal(Vec3::new(0.8, 0.6, 0.2), 0.0);
    let world = if false {
        let mut world = Scene::new();
        world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.), mat_ground);
        world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5), mat_center);
        world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5), mat_left);
        world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5), mat_right);
        world
    } else {
        random_scene(&mut rng)
    };

    println!("Generated, rendering...");

    let samples = 100;
    let sample_weight = 1. / (samples as f32);
    let max_depth = 50;

    // Render
    for y in 0..height {
        for x in 0..width {
            let mut sum = RGBf32::default();
            for _ in 0..samples {
                let u = (x as f32) + rand_f32(&mut rng);
                let v = (y as f32) + rand_f32(&mut rng);
                let r = camera.cast(u, v, sample_weight);
                sum += RGBf32::from(world.ray_color(&r, &mut rng, max_depth));
            }
            fbuf[(x, y)] = sum.gamma_correct();
        }
    }

    println!("Writing result...");
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
