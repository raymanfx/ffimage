use core::ops::{Deref, DerefMut};

use ffimage::traits::Pixel;

use crate::yuv::*;

/// YUV 4:2:2 format
pub type Yuyv<T> = Yuv422<T, 0, 2, 1, 3>;
pub type Uyvy<T> = Yuv422<T, 1, 3, 0, 2>;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yuv422<
    T,
    const Y0: usize = 0,
    const Y1: usize = 1,
    const U: usize = 2,
    const V: usize = 3,
>(pub [T; 4]);

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> Deref
    for Yuv422<T, Y0, Y1, U, V>
{
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> DerefMut
    for Yuv422<T, Y0, Y1, U, V>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> Pixel
    for Yuv422<T, Y0, Y1, U, V>
where
    T: Copy,
{
    type T = T;

    fn channels() -> u8 {
        4
    }

    fn subpixels() -> u8 {
        2
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize>
    From<Yuv422<T, Y0, Y1, U, V>> for [Yuv<T>; 2]
where
    T: Copy,
{
    fn from(pix: Yuv422<T, Y0, Y1, U, V>) -> Self {
        let sub1 = Yuv {
            0: [pix[Y0], pix[U], pix[V]],
        };
        let sub2 = Yuv {
            0: [pix[Y1], pix[U], pix[V]],
        };

        [sub1, sub2]
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> From<[Yuv<T>; 2]>
    for Yuv422<T, Y0, Y1, U, V>
where
    T: Copy + Default,
{
    fn from(pix: [Yuv<T>; 2]) -> Self {
        let mut yuv422: Yuv422<T, Y0, Y1, U, V> = Yuv422::default();
        yuv422[Y0] = pix[0][0];
        yuv422[U] = pix[0][1];
        yuv422[Y1] = pix[1][0];
        yuv422[V] = pix[1][2];
        yuv422
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Yuv422::<u8, 0, 2, 1, 3>::channels(), 4);
    }
}
