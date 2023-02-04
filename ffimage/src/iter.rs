use core::marker::PhantomData;

/// Adapter which converts a bytestream into a typed pixel stream.
///
/// The trait is automatically implemented for all pixel types which implement the `From<[T; C]>`
/// trait where T: Copy and C means the number of channels (e.g. 3 for RGB).
pub trait PixelsExt: Iterator {
    fn pixels<P, const C: usize>(self) -> Pixels<Self::Item, Self, P, C>
    where
        Self: Sized,
    {
        Pixels::new(self)
    }
}

impl<I> PixelsExt for I where I: Iterator {}

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
        for channel in chunk.iter_mut().take(C).skip(1) {
            *channel = self.iter.next()?
        }
        Some(P::from(chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgb;

    #[test]
    fn pixels() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut pixels = buf.iter().copied().pixels();

        assert_eq!(pixels.next(), Some(Rgb::<u8>([1, 2, 3])));
        assert_eq!(pixels.next(), Some(Rgb::<u8>([4, 5, 6])));
        assert_eq!(pixels.next(), Some(Rgb::<u8>([7, 8, 9])));
        assert_eq!(pixels.next(), None);
    }
}
