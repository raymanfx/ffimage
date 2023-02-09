use std::ops::Deref;

use ffimage::color::{Gray, Rgb};

pub struct Rgba<T>([T; 4]);

impl<T> Deref for Rgba<T> {
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<[T; 4]> for Rgba<T> {
    fn from(value: [T; 4]) -> Self {
        Rgba(value)
    }
}

impl From<Rgb<u8>> for Rgba<u8> {
    fn from(value: Rgb<u8>) -> Self {
        Rgba([value[0], value[1], value[2], 255u8])
    }
}

impl From<Gray<u8>> for Rgba<u8> {
    fn from(value: Gray<u8>) -> Self {
        Rgba([value[0], value[0], value[0], 255u8])
    }
}

impl<T: Copy> From<Rgba<T>> for Rgb<T> {
    fn from(value: Rgba<T>) -> Self {
        Rgb([value[0], value[1], value[2]])
    }
}
