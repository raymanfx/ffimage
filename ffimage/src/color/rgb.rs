use core::ops::{Deref, DerefMut};

use crate::Pixel;

/// RGB pixel
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb<T, const R: usize = 0, const G: usize = 1, const B: usize = 2>(pub [T; 3]);

/// BGR pixel
pub type Bgr<T> = Rgb<T, 2, 1, 0>;

impl<T, const R: usize, const G: usize, const B: usize> From<[T; 3]> for Rgb<T, R, G, B> {
    fn from(value: [T; 3]) -> Self {
        Rgb(value)
    }
}

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

impl<T, const R: usize, const G: usize, const B: usize> Pixel for Rgb<T, R, G, B> {
    const CHANNELS: u8 = 3;
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
        Rgb::<T, 2, 1, 0>([T::from(rgb[2]), T::from(rgb[1]), T::from(rgb[0])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Rgb::<u8>::CHANNELS, 3);
    }

    #[test]
    fn index_mut() {
        let pix: Rgb<u8> = Rgb { 0: [1, 2, 3] };

        assert_eq!(pix.0[0], 1);
        assert_eq!(pix.0[1], 2);
        assert_eq!(pix.0[2], 3);
    }
}
