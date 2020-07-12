use std::array;
use std::convert::TryFrom;

use num_traits::AsPrimitive;

use crate::color::gray::*;
use crate::color::rgb::*;
use crate::core::traits::{Pixel, StorageType};
use crate::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_pix_to_pix3 {
    ($src:ident, $dst:ident, $_0:expr, $_1:expr, $_2:expr) => {
        impl<I: StorageType + AsPrimitive<O>, O: StorageType + 'static> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                $dst {
                    0: [pix[$_0].as_(), pix[$_1].as_(), pix[$_2].as_()],
                }
            }
        }
    };
}

macro_rules! impl_from_pix_to_pix4 {
    ($src:ident, $dst:ident, $_0:expr, $_1:expr, $_2:expr) => {
        impl<I: StorageType + AsPrimitive<O>, O: StorageType + 'static> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                $dst {
                    0: [pix[$_0].as_(), pix[$_1].as_(), pix[$_2].as_(), O::zero()],
                }
            }
        }
    };
}

create_pixel!(Bgr, 3, #[doc = "BGR pixel"]);
create_pixel!(Bgra, 4, #[doc = "BGR pixel with alpha"]);

impl_from_pix_to_pix4!(Bgr, Bgra, 0, 1, 2);
impl_from_pix_to_pix3!(Bgra, Bgr, 0, 1, 2);

impl_from_pix_to_pix3!(Gray, Bgr, 0, 0, 0);
impl_from_pix_to_pix4!(Gray, Bgra, 0, 0, 0);

impl_from_pix_to_pix3!(Rgb, Bgr, 2, 1, 0);
impl_from_pix_to_pix4!(Rgb, Bgra, 2, 1, 0);
impl_from_pix_to_pix3!(Rgba, Bgr, 2, 1, 0);
impl_from_pix_to_pix4!(Rgba, Bgra, 2, 1, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let pix: Bgr<u8> = Bgr { 0: [255; 3] };

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn try_from() {
        let mem = vec![255; 3];
        let pix: Bgr<u8> = Pixel::try_from(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn channels() {
        assert_eq!(Bgr::<u8>::channels(), 3);
        assert_eq!(Bgra::<u8>::channels(), 4);
    }
}
