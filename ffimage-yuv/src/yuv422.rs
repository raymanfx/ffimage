use ffimage::packed::traits::ConvertSlice;
use ffimage::traits::Pixel;

use crate::yuv::*;

/// YUV 4:2:2 format
pub type Yuyv<T> = Yuv422<T, 0, 2, 1, 3>;
pub type Uyvy<T> = Yuv422<T, 1, 3, 0, 2>;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yuv422<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize>(pub [T; 4]);

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> Yuv422<T, Y0, Y1, U, V> {
    /// Returns a new pixel
    ///
    /// # Arguments
    ///
    /// * `channels` - Channel values
    pub fn new(channels: [T; 4]) -> Self {
        Yuv422 { 0: channels }
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> From<[T; 4]>
    for Yuv422<T, Y0, Y1, U, V>
where
    T: Copy,
{
    fn from(array: [T; 4]) -> Self {
        Yuv422 { 0: array }
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

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> core::ops::Index<usize>
    for Yuv422<T, Y0, Y1, U, V>
{
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> core::ops::IndexMut<usize>
    for Yuv422<T, Y0, Y1, U, V>
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
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

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize> ConvertSlice<Yuv<T>>
    for Yuv422<T, Y0, Y1, U, V>
where
    T: Copy,
{
    fn convert<IT: AsRef<[Self]>, OT: AsMut<[Yuv<T>]>>(input: IT, mut output: OT) {
        for (outp, inp) in output
            .as_mut()
            .chunks_exact_mut(2)
            .zip(input.as_ref().iter())
        {
            let yuv = <[Yuv<T>; 2]>::from(*inp);
            outp[0] = yuv[0];
            outp[1] = yuv[1];
        }
    }
}

impl<T, const Y0: usize, const Y1: usize, const U: usize, const V: usize>
    ConvertSlice<Yuv422<T, Y0, Y1, U, V>> for Yuv<T>
where
    T: Copy + Default,
{
    fn convert<IT: AsRef<[Self]>, OT: AsMut<[Yuv422<T, Y0, Y1, U, V>]>>(input: IT, mut output: OT) {
        for (outp, inp) in output
            .as_mut()
            .iter_mut()
            .zip(input.as_ref().chunks_exact(2))
        {
            *outp = Yuv422::<T, Y0, Y1, U, V>::from([inp[0], inp[1]]);
        }
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
