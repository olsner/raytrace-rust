#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
pub struct RGB<C> {
    pub r : C,
    pub g : C,
    pub b : C
}

impl<C : From<u8>> RGB<C> {
    pub fn black() -> Self {
        Self{ r : C::from(0u8),
              g : C::from(0u8),
              b : C::from(0u8) }
    }

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

