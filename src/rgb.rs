use std::ops::AddAssign;
use std::ops::Mul;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
pub struct RGB<C> {
    pub r : C,
    pub g : C,
    pub b : C
}

impl<C : From<u8>> RGB<C> {
    #[allow(dead_code)]
    pub fn black() -> Self {
        Self{ r : C::from(0u8),
              g : C::from(0u8),
              b : C::from(0u8) }
    }

    #[allow(dead_code)]
    pub fn new(r : C, g : C, b : C) -> Self {
        Self{ r, g, b }
    }
}

pub type RGBu8 = RGB<u8>;
pub type RGBf32 = RGB<f32>;

impl From<RGBf32> for RGBu8 {
    fn from(f : RGBf32) -> RGBu8 {
        RGBu8 { r : (255.0 * f.r).round() as u8,
                g : (255.0 * f.g).round() as u8,
                b : (255.0 * f.b).round() as u8 }
    }
}

// Fancy generics, seems this is ambiguous because it doesn't exclude S==D
// which is also in core (or std or wherever).
//
//impl<D, S> From<RGB<S>> for RGB<D> where D : From<S> {
//    fn from(s : RGB<S>) -> RGB<D> {
//        RGBu8 { r : D::from(s.r),
//                g : D::from(s.g),
//                b : D::from(s.b) }
//    }
//}

impl<U, T : AddAssign<U>> AddAssign<RGB<U>> for RGB<T> {
    fn add_assign(self: &mut Self, other: RGB<U>) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl<T : Mul + Mul<Output = T>> Mul for RGB<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b }
    }
}

impl<T : Copy + Mul + Mul<Output = T>> Mul<T> for RGB<T> {
    type Output = Self;
    fn mul(self, scale: T) -> Self {
        Self {
            r: self.r * scale,
            g: self.g * scale,
            b: self.b * scale }
    }
}

