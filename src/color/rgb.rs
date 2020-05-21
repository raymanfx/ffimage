use std::array;
use std::convert::TryFrom;

use num_traits::AsPrimitive;

use crate::color::bgr::*;
use crate::color::gray::*;
use crate::core::traits::{Pixel, StorageType};
use crate::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_self_ref {
    ($src:ident, $dst:ident) => {
        impl<I: StorageType + AsPrimitive<O>, O: StorageType + 'static> From<&$src<I>> for $dst<O> {
            fn from(pix: &$src<I>) -> Self {
                Self::from(*pix)
            }
        }
    };
}

macro_rules! impl_from_pix_to_pix3 {
    ($src:ident, $dst:ident, $_0:expr, $_1:expr, $_2:expr) => {
        impl<I: StorageType + AsPrimitive<O>, O: StorageType + 'static> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                $dst {
                    0: [pix[$_0].as_(), pix[$_1].as_(), pix[$_2].as_()],
                }
            }
        }

        impl_from_self_ref!($src, $dst);
    };
}

macro_rules! impl_from_pix_to_pix4 {
    ($src:ident, $dst:ident, $_0:expr, $_1:expr, $_2:expr) => {
        impl<I: StorageType + AsPrimitive<O>, O: StorageType + 'static> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                $dst {
                    0: [pix[$_0].as_(), pix[$_1].as_(), pix[$_2].as_(), O::default()],
                }
            }
        }

        impl_from_self_ref!($src, $dst);
    };
}

create_pixel!(Rgb, 3, #[doc = "RGB pixel"]);
create_pixel!(Rgba, 4, #[doc = "RGB pixel with alpha"]);

impl_from_pix_to_pix4!(Rgb, Rgba, 0, 1, 2);
impl_from_pix_to_pix3!(Rgba, Rgb, 0, 1, 2);

impl_from_pix_to_pix3!(Gray, Rgb, 0, 0, 0);
impl_from_pix_to_pix4!(Gray, Rgba, 0, 0, 0);

impl_from_pix_to_pix3!(Bgr, Rgb, 2, 1, 0);
impl_from_pix_to_pix4!(Bgr, Rgba, 2, 1, 0);
impl_from_pix_to_pix3!(Bgra, Rgb, 2, 1, 0);
impl_from_pix_to_pix4!(Bgra, Rgba, 2, 1, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let pix: Rgb<u8> = Rgb { 0: [255; 3] };

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn cast_from_slice() {
        let mem = vec![255; 3];
        let pix = Rgb::<u8>::cast_from_slice(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn cast_from_slice_mut() {
        let mut mem = vec![255; 3];
        let pix = Rgb::<u8>::cast_from_slice_mut(&mut mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn try_from() {
        let mem = vec![255; 3];
        let pix: Rgb<u8> = Pixel::try_from(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn channels() {
        assert_eq!(Rgb::<u8>::channels(), 3);
        assert_eq!(Rgba::<u8>::channels(), 4);
    }
}
