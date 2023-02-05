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

impl<'a, T, I, P, const C: usize> Pixels<T, I, P, C>
where
    T: Copy + 'a,
    I: Iterator<Item = T>,
    P: From<[T; C]>,
{
    pub fn write<P2>(self, mut out: impl Iterator<Item = &'a mut T>)
    where
        P2: From<P> + Deref,
        <P2 as Deref>::Target: AsRef<[T]>,
    {
        self.map(|p| P2::from(p)).for_each(|p2| {
            p2.as_ref().iter().for_each(|t| {
                let _out = out.next().unwrap();
                *_out = *t;
            });
        });
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
        for channel in chunk.iter_mut().take(C).skip(1) {
            *channel = self.iter.next()?
        }
        Some(P::from(chunk))
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
            .write::<Bgr<u8>>(out.iter_mut());
        assert_eq!(out, [3, 2, 1, 6, 5, 4, 9, 8, 7]);
    }
}
