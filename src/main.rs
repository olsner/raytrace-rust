#![feature(test)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

mod rgb;
mod framebuf;

use rgb::RGBu8;
use rgb::RGBf32;
use framebuf::Framebuf;

extern crate nalgebra as na;
use na::{UnitVector3, Vector3, Point3};
type Vec3 = Vector3<f32>;
type UVec3 = UnitVector3<f32>;

struct Ray {
    origin : Point3<f32>,
    direction : UVec3,
}

impl Ray {
    fn at(self : &Self, t : f32) -> Point3<f32> {
        self.origin + (t * self.direction.into_inner())
    }
}

impl From<Vec3> for RGBf32 {
    fn from(vec : Vec3) -> RGBf32 {
        RGBf32{ r : vec.x, g : vec.y, b : vec.z }
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Sphere {
    center : Point3<f32>,
    radius : f32,
}

impl Sphere {
    fn new(center : Point3<f32>, radius : f32) -> Sphere {
        Sphere{ center, radius }
    }
}

struct HitRecord {
    point : Point3<f32>,
    normal : UVec3,
    distance : f32,
}

trait Shape {
    fn hit(self : &Self, ray : &Ray) -> Option<HitRecord>;
}

impl Shape for Sphere {
    fn hit(self : &Sphere, r : &Ray) -> Option<HitRecord> {
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

fn best(left : Option<HitRecord>, right : Option<HitRecord>) -> Option<HitRecord> {
    match left {
        None => right,
        Some(lhit) => Some(match right {
            Some(rhit) =>
                if lhit.distance < rhit.distance { lhit } else { rhit }
            None => lhit,
        })
    }
}

struct Scene {
    spheres : Vec<Sphere>,
}

impl Scene {
    fn new() -> Self {
        Scene { spheres : Vec::new() }
    }

    fn add(self : &mut Self, sphere : Sphere) {
        self.spheres.push(sphere);
    }
}

impl Shape for Scene {
    fn hit(self : &Scene, r : &Ray) -> Option<HitRecord> {
        let mut best_hit = None;
        for sphere in &self.spheres {
            best_hit = best(sphere.hit(r), best_hit);
        }
        return best_hit;
    }
}

fn ray_color(world : &impl Shape, ray : &Ray) -> RGBf32 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    match world.hit(ray) {
        Some(hit) => {
            let n = hit.normal.into_inner();
            return RGBf32::from(0.5 * (n + Vec3::repeat(1.0)));
        }
        None => ()
    }

    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);
    RGBf32::from(white.lerp(&blue, t))
}

fn write_ppm<Pixel>(path : &Path, buf : &Framebuf<Pixel>)
  where RGBu8 : From<Pixel>, Pixel : Copy {
    print!("Writing {}\n", path.display());
    let mut file = File::create(&path).unwrap();
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

    let aspect_ratio = width as f32 / height as f32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. -
        Vec3::new(0., 0., focal_length);

    let mut world = Scene::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.));

    // Render
    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let v = y as f32 / (height - 1) as f32;

            let direction = UVec3::new_normalize(lower_left_corner + u*horizontal + v*vertical - origin);
            let r = Ray { origin, direction };
            fbuf[(x, y)] = ray_color(&world, &r);
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
