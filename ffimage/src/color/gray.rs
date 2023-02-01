use core::ops::{Deref, DerefMut};

use num::FromPrimitive;

use crate::{color::rgb::Rgb, Pixel};

/// Grayscale pixel
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Gray<T>(pub [T; 1]);

impl<T> Deref for Gray<T> {
    type Target = [T; 1];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Gray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Pixel for Gray<T> {
    fn channels() -> u8 {
        1
    }

    fn subpixels() -> u8 {
        1
    }
}

impl<T, U, const R: usize, const G: usize, const B: usize> From<Rgb<U, R, G, B>> for Gray<T>
where
    T: Copy + FromPrimitive,
    U: Copy + Into<f32>,
{
    fn from(rgb: Rgb<U, R, G, B>) -> Self {
        // rec601 luma
        let y =
            T::from_f32(0.2126 * rgb[R].into() + 0.7152 * rgb[G].into() + 0.0722 * rgb[B].into())
                .expect("could not convert from f32 to T");

        Gray([y])
    }
}

impl<T, const R: usize, const G: usize, const B: usize> From<Gray<T>> for Rgb<T, R, G, B>
where
    T: Copy,
{
    fn from(gray: Gray<T>) -> Self {
        Rgb([gray[0], gray[0], gray[0]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Gray::<u8>::channels(), 1);
    }

    #[test]
    fn index_mut() {
        let pix: Gray<u8> = Gray { 0: [1] };

        assert_eq!(pix[0], 1);
    }
}
