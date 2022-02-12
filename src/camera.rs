use crate::Point3;
use crate::Vec3;
use crate::UVec3;

use crate::ray::Ray;

pub struct Camera {
    origin : Point3<f32>,
    lower_left : Point3<f32>,
    horizontal : Vec3,
    vertical : Vec3
}

impl Camera {
    pub fn new(width : u32, height : u32) -> Camera {
        let fwidth = width as f32;
        let fheight = height as f32;
        let aspect_ratio = fwidth / fheight;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0., 0., 0.);
        let mut horizontal = Vec3::new(viewport_width, 0., 0.);
        let mut vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left = origin - horizontal / 2. - vertical / 2. -
            Vec3::new(0., 0., focal_length);

        horizontal /= fwidth - 1.0;
        vertical /= fheight - 1.0;

        Camera { origin, lower_left, horizontal, vertical }
    }

    pub fn look_from_at(origin : Point3<f32>, lookat : Point3<f32>, up : UVec3,
            vfov : f32, width : u32, height : u32) -> Camera {
        let fwidth = width as f32;
        let fheight = height as f32;
        let h = (vfov.to_radians() / 2.0).tan();

        let viewport_height = 2.0 * h;
        let aspect_ratio = fwidth / fheight;
        let viewport_width = aspect_ratio * viewport_height;

        let w = UVec3::new_normalize(origin - lookat);
        let u = up.cross(&w);
        let v = w.cross(&u);

        let mut horizontal = viewport_width * u;
        let mut vertical = viewport_height * v;
        let lower_left = origin - horizontal / 2. - vertical / 2. - w.into_inner();

        horizontal /= fwidth - 1.0;
        vertical /= fheight - 1.0;

        Camera { origin, lower_left, horizontal, vertical }
    }

    pub fn cast(&self, u : f32, v : f32, weight : f32) -> Ray {
        let direction = self.lower_left + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new_normalize(self.origin, direction, Vec3::repeat(weight))
    }
}
