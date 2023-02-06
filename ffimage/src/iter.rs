use core::{marker::PhantomData, ops::Deref};

/// Adapter which converts a bytestream into a typed pixel stream.
///
/// The trait is automatically implemented for all pixel types which implement the `From<[T; C]>`
/// trait where T: Copy and C means the number of channels (e.g. 3 for RGB).
pub trait PixelsExt<const C: usize>: Iterator {
    fn pixels<P>(self) -> Pixels<Self::Item, Self, P, P, C, C>
    where
        Self: Sized,
    {
        Pixels::new(self)
    }
}

impl<I, const C: usize> PixelsExt<C> for I where I: Iterator {}

pub struct Pixels<T, I, P, P2, const C: usize, const C2: usize> {
    _marker: PhantomData<(T, P, P2)>,
    iter: I,
}

impl<T, I, P, P2, const C: usize, const C2: usize> Pixels<T, I, P, P2, C, C2> {
    pub fn new(iter: I) -> Self {
        Pixels {
            _marker: PhantomData,
            iter,
        }
    }
}

impl<'a, T, I, P, P2, const C: usize, const C2: usize> Pixels<T, I, P, P2, C, C2>
where
    T: Copy + 'a,
    I: Iterator<Item = T>,
    P: From<[T; C]>,
    P2: From<P>,
{
    pub fn write(self, out: impl IntoIterator<Item = &'a mut T>)
    where
        P2: Deref<Target = [T; C2]>,
    {
        let mut out = out.into_iter();

        self.for_each(|p2| {
            p2.iter().for_each(|t| *(out.next().unwrap()) = *t);
        });
    }
}

impl<'a, T, I, P, P2, const C: usize, const C2: usize> Pixels<T, I, P, P2, C, C2>
where
    T: Copy + Default + 'a,
    I: Iterator<Item = T>,
    P: From<[T; C]>,
{
    pub fn colorconvert<P3>(self) -> Pixels<T, I, P, P3, C, C2>
    where
        P3: From<[T; C2]> + From<P2>,
    {
        Pixels::new(self.iter)
    }
}

impl<T, I, P, P2, const C: usize, const C2: usize> Iterator for Pixels<T, I, P, P2, C, C2>
where
    T: Copy,
    I: Iterator<Item = T>,
    P: From<[T; C]>,
    P2: From<P>,
{
    type Item = P2;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = [self.iter.next()?; C];
        for i in 1..C {
            chunk[i] = self.iter.next()?;
        }
        Some(P2::from(P::from(chunk)))
    }
}

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
