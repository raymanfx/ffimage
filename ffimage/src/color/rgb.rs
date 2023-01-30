use core::ops::{Deref, DerefMut};

use num::FromPrimitive;

use crate::traits::Pixel;

/// RGB pixel
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb<T, const R: usize = 0, const G: usize = 1, const B: usize = 2>(pub [T; 3]);

impl<T, const R: usize, const G: usize, const B: usize> Deref for Rgb<T, R, G, B> {
    type Target = [T; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const R: usize, const G: usize, const B: usize> DerefMut for Rgb<T, R, G, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const R: usize, const G: usize, const B: usize> Pixel for Rgb<T, R, G, B>
where
    T: Copy,
{
    type T = T;

    fn channels() -> u8 {
        3
    }

    fn subpixels() -> u8 {
        1
    }
}

impl<T, U> From<Rgb<U, 2, 1, 0>> for Rgb<T, 0, 1, 2>
where
    T: Copy + Default + From<U>,
    U: Copy,
{
    fn from(rgb: Rgb<U, 2, 1, 0>) -> Self {
        Rgb::<T, 0, 1, 2>([T::from(rgb[2]), T::from(rgb[1]), T::from(rgb[0])])
    }
}

impl<T, U> From<Rgb<U, 0, 1, 2>> for Rgb<T, 2, 1, 0>
where
    T: Copy + Default + From<U>,
    U: Copy,
{
    fn from(rgb: Rgb<U, 0, 1, 2>) -> Self {
        Rgb::<T, 2, 1, 0>([T::from(rgb[0]), T::from(rgb[1]), T::from(rgb[2])])
    }
}

impl<
        T,
        U,
        const R: usize,
        const G: usize,
        const B: usize,
        const RGBA_R: usize,
        const RGBA_G: usize,
        const RGBA_B: usize,
        const A: usize,
    > From<Rgba<U, RGBA_R, RGBA_G, RGBA_B, A>> for Rgb<T, R, G, B>
where
    T: Copy + Default + From<U>,
    U: Copy,
{
    fn from(rgba: Rgba<U, RGBA_R, RGBA_G, RGBA_B, A>) -> Self {
        let mut rgb = Rgb::<T, R, G, B>::default();
        rgb[R] = T::from(rgba[RGBA_R]);
        rgb[G] = T::from(rgba[RGBA_G]);
        rgb[B] = T::from(rgba[RGBA_B]);
        rgb
    }
}

/// RGB pixel with alpha channel
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgba<T, const R: usize = 0, const G: usize = 1, const B: usize = 2, const A: usize = 3>(
    pub [T; 4],
);

impl<T, const R: usize, const G: usize, const B: usize, const A: usize> Deref
    for Rgba<T, R, G, B, A>
{
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const R: usize, const G: usize, const B: usize, const A: usize> DerefMut
    for Rgba<T, R, G, B, A>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const R: usize, const G: usize, const B: usize, const A: usize> Pixel
    for Rgba<T, R, G, B, A>
where
    T: Copy,
{
    type T = T;

    fn channels() -> u8 {
        4
    }

    fn subpixels() -> u8 {
        1
    }
}

impl<T, U> From<Rgba<U, 2, 1, 0, 3>> for Rgba<T, 0, 1, 2, 3>
where
    T: Copy + Default + From<U>,
    U: Copy,
{
    fn from(rgba: Rgba<U, 2, 1, 0>) -> Self {
        Rgba::<T, 0, 1, 2, 3>([
            T::from(rgba[2]),
            T::from(rgba[1]),
            T::from(rgba[0]),
            T::from(rgba[3]),
        ])
    }
}

impl<T, U> From<Rgba<U, 0, 1, 2, 3>> for Rgba<T, 2, 1, 0, 3>
where
    T: Copy + Default + From<U>,
    U: Copy,
{
    fn from(rgba: Rgba<U, 0, 1, 2>) -> Self {
        Rgba::<T, 2, 1, 0, 3>([
            T::from(rgba[0]),
            T::from(rgba[1]),
            T::from(rgba[2]),
            T::from(rgba[3]),
        ])
    }
}

impl<
        T,
        U,
        const RGB_R: usize,
        const RGB_G: usize,
        const RGB_B: usize,
        const R: usize,
        const G: usize,
        const B: usize,
        const A: usize,
    > From<Rgb<U, RGB_R, RGB_G, RGB_B>> for Rgba<T, R, G, B, A>
where
    T: Copy + Default + From<U> + FromPrimitive,
    U: Copy,
{
    fn from(rgb: Rgb<U, RGB_R, RGB_G, RGB_B>) -> Self {
        let mut rgba = Rgba::<T, R, G, B, A>::default();
        rgba[R] = T::from(rgb[RGB_R]);
        rgba[G] = T::from(rgb[RGB_G]);
        rgba[B] = T::from(rgb[RGB_B]);
        rgba[A] = T::from_u8(255).unwrap();
        rgba
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Rgb::<u8>::channels(), 3);
        assert_eq!(Rgba::<u8>::channels(), 4);
    }

    #[test]
    fn index_mut() {
        let pix: Rgb<u8> = Rgb { 0: [1, 2, 3] };

        assert_eq!(pix.0[0], 1);
        assert_eq!(pix.0[1], 2);
        assert_eq!(pix.0[2], 3);
    }
}
