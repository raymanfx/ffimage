use core::{marker::PhantomData, ops::Deref};

/// Adapter which converts a bytestream into a typed pixel stream.
///
/// The trait is automatically implemented for all pixel types which implement the `From<[T; C]>`
/// trait where T: Copy and C means the number of channels (e.g. 3 for RGB).
pub trait PixelsExt<const C: usize>: Iterator {
    fn pixels<P>(self) -> Pixels<Self::Item, Self, P, C>
    where
        Self: Sized,
    {
        Pixels::new(self)
    }
}

impl<I, const C: usize> PixelsExt<C> for I where I: Iterator {}

pub struct Pixels<T, I, P, const C: usize> {
    _marker: PhantomData<(T, P)>,
    iter: I,
}

impl<T, I, P, const C: usize> Pixels<T, I, P, C> {
    pub fn new(iter: I) -> Self {
        Pixels {
            _marker: PhantomData,
            iter,
        }
    }
}

impl<T, I, P, const C: usize> Iterator for Pixels<T, I, P, C>
where
    T: Copy,
    I: Iterator<Item = T>,
    P: From<[T; C]>,
{
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = [self.iter.next()?; C];
        for i in 1..C {
            chunk[i] = self.iter.next()?;
        }
        Some(P::from(chunk))
    }
}

/// Adapter which converts between color formats.
///
/// The trait is automatically implemented for all pixel types which implement the `From<[T; C]>`
/// trait where T: Copy and C means the number of channels (e.g. 3 for RGB).
pub trait ColorConvertExt: Iterator {
    fn colorconvert<P2>(self) -> ColorConvert<Self, Self::Item, P2>
    where
        Self: Sized,
        P2: From<Self::Item>,
    {
        ColorConvert::new(self)
    }
}

impl<I> ColorConvertExt for I where I: Iterator {}

pub struct ColorConvert<I, P, P2> {
    _marker: PhantomData<(P, P2)>,
    iter: I,
}

impl<I, P, P2> ColorConvert<I, P, P2> {
    pub fn new(iter: I) -> Self {
        ColorConvert {
            _marker: PhantomData,
            iter,
        }
    }
}

impl<I, P, P2> Iterator for ColorConvert<I, P, P2>
where
    P2: From<P>,
    I: Iterator<Item = P>,
{
    type Item = P2;

    fn next(&mut self) -> Option<Self::Item> {
        Some(P2::from(self.iter.next()?))
    }
}

/// Adapter which converts a typed pixel stream into a bytestream.
///
/// The trait is automatically implemented for all pixel types which implement the
/// `Deref<Target = [T; C]>` trait where T: Copy and C means the number of channels
/// (e.g. 3 for RGB).
pub trait WriteExt<'a, T, O, const C: usize>: Iterator {
    fn write(self, out: O)
    where
        Self: Sized,
        Self::Item: Deref<Target = [T; C]>,
        T: 'a + Copy,
        O: IntoIterator<Item = &'a mut T>,
    {
        let mut out = out.into_iter();
        self.for_each(|chunk| {
            for i in 0..C {
                *(out.next().unwrap()) = chunk[i];
            }
        });
    }
}

impl<'a, T, I, O, const C: usize> WriteExt<'a, T, O, C> for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Bgr, Rgb};

    #[test]
    fn pixels() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut pixels = buf.iter().copied().pixels();

        assert_eq!(pixels.next(), Some(Rgb::<u8>([1, 2, 3])));
        assert_eq!(pixels.next(), Some(Rgb::<u8>([4, 5, 6])));
        assert_eq!(pixels.next(), Some(Rgb::<u8>([7, 8, 9])));
        assert_eq!(pixels.next(), None);
    }

    #[test]
    fn write() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut out = [0; 9];
        buf.iter()
            .copied()
            .pixels::<Rgb<u8>>()
            .colorconvert::<Bgr<u8>>()
            .write(&mut out);
        assert_eq!(out, [3, 2, 1, 6, 5, 4, 9, 8, 7]);
    }
}
