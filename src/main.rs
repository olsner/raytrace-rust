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

impl From<Vec3> for RGBf32 {
    fn from(vec : Vec3) -> RGBf32 {
        RGBf32{ r : vec.x, g : vec.y, b : vec.z }
    }
}

fn ray_color(ray : &Ray) -> RGBf32 {
    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    RGBf32::from(white.lerp(&blue, t))
}

fn write_ppm<Pixel>(path : &Path, buf : &Framebuf<Pixel>)
  where RGBu8 : From<Pixel>, Pixel : Copy {
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

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let v = y as f32 / (width - 1) as f32;

            let direction = UVec3::new_normalize(lower_left_corner + u*horizontal + v*vertical - origin);
            let r = Ray { origin, direction };
            fbuf[(x, y)] = ray_color(&r);
        }
    }

    write_ppm(Path::new("frame.ppm"), &fbuf);
}
