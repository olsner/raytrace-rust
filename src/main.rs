use std::fs::File;
use std::io::Write;
use std::path::Path;

mod rgb;
mod framebuf;

use rgb::RGBu8;
use rgb::RGBf32;
use framebuf::Framebuf;

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
    let mut buf = Framebuf::new(width, height, RGBu8::default());
    let mut fbuf = Framebuf::new(width, height, RGBf32::default());

    for y in (0..height).rev() {
        for x in 0..width {
            let r = x as f32 / (width - 1) as f32;
            let g = y as f32 / (width - 1) as f32;
            let b = 0.25f32;

            let ir = (255.0 * r).round() as u8;
            let ig = (255.0 * g).round() as u8;
            let ib = (255.0 * b).round() as u8;

            buf[(x, y)] = RGBu8{ r : ir, g : ig, b : ib };
            fbuf[(x, y)] = RGBf32{ r : r, g : g, b : b };
        }
    }

    write_ppm(Path::new("frame_u8.ppm"), &buf);
    write_ppm(Path::new("frame_f32.ppm"), &fbuf);
}
