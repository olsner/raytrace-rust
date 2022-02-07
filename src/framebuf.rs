use std::ops::Index;
use std::ops::IndexMut;
use std::vec::Vec;

pub struct Framebuf<Pixel> {
    pub width : u32,
    pub height : u32,
    buffer: Vec<Pixel>,
}

impl<Pixel : Clone> Framebuf<Pixel> {
    pub fn new(width : u32, height : u32, init : Pixel) -> Self {
        let bufsize = (width as usize) * (height as usize);
        let mut vec = Vec::new();
        vec.resize(bufsize, init);
        Self{ width, height, buffer : vec }
    }
}

impl<Pixel> Framebuf<Pixel> {
    fn flat_ix(&self, index: Ix2D) -> usize {
        index.1 as usize * self.width as usize + index.0 as usize
    }
}

type Ix2D = (u32, u32);

impl<T> Index<Ix2D> for Framebuf<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Ix2D) -> &T {
        &self.buffer[self.flat_ix(index)]
    }
}

impl<T> IndexMut<Ix2D> for Framebuf<T> {
    #[inline]
    fn index_mut(&mut self, index: Ix2D) -> &mut T {
        let i = self.flat_ix(index);
        &mut self.buffer[i]
    }
}

