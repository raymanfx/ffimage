use core::{
    cmp::Ord,
    ops::{Deref, DerefMut},
};

use num_traits::{AsPrimitive, FromPrimitive};

use ffimage::color::rgb::*;
use ffimage::Pixel;

/// YUV pixel
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Yuv<T, const Y: usize = 0, const U: usize = 1, const V: usize = 2>(pub [T; 3]);

impl<T, const Y: usize, const U: usize, const V: usize> Deref for Yuv<T, Y, U, V> {
    type Target = [T; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const Y: usize, const U: usize, const V: usize> DerefMut for Yuv<T, Y, U, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const Y: usize, const U: usize, const V: usize> Pixel for Yuv<T, Y, U, V> {
    fn channels() -> u8 {
        3
    }

    fn subpixels() -> u8 {
        1
    }
}

impl<
        T,
        const Y: usize,
        const U: usize,
        const V: usize,
        const R: usize,
        const G: usize,
        const B: usize,
    > From<Rgb<T, R, G, B>> for Yuv<T, Y, U, V>
where
    T: Copy + Default + AsPrimitive<i32> + FromPrimitive,
{
    fn from(rgb: Rgb<T, R, G, B>) -> Self {
        let r = rgb[R].as_();
        let g = rgb[G].as_();
        let b = rgb[B].as_();

        let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
        let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
        let v = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;

        let mut yuv = Yuv::<T, Y, U, V>::default();
        yuv[Y] = T::from_i32(y).unwrap();
        yuv[U] = T::from_i32(u).unwrap();
        yuv[V] = T::from_i32(v).unwrap();
        yuv
    }
}

impl<
        T,
        const R: usize,
        const G: usize,
        const B: usize,
        const Y: usize,
        const U: usize,
        const V: usize,
    > From<Yuv<T, Y, U, V>> for Rgb<T, R, G, B>
where
    T: Copy + Default + AsPrimitive<i32> + FromPrimitive,
{
    fn from(yuv: Yuv<T, Y, U, V>) -> Self {
        let y = yuv[Y].as_();
        let u = yuv[U].as_();
        let v = yuv[V].as_();
        let c = y - 16;
        let d = u - 128;
        let e = v - 128;

        let r = ((298 * c + 409 * e + 128) >> 8).clamp(0, 255);
        let g = ((298 * c - 100 * d - 208 * e + 128) >> 8).clamp(0, 255);
        let b = ((298 * c + 516 * d + 128) >> 8).clamp(0, 255);

        let mut rgb = Rgb::<T, R, G, B>::default();
        rgb[R] = T::from_i32(r).unwrap();
        rgb[G] = T::from_i32(g).unwrap();
        rgb[B] = T::from_i32(b).unwrap();
        rgb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Yuv::<u8>::channels(), 3);
    }
}
