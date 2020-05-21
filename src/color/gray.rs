use std::array;
use std::convert::TryFrom;

use num_traits::{AsPrimitive, FromPrimitive};

use crate::color::bgr::*;
use crate::color::rgb::*;
use crate::core::traits::{Pixel, StorageType};
use crate::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_self_ref {
    ($src:ident, $dst:ident) => {
        impl<I: StorageType + AsPrimitive<f32>, O: StorageType + FromPrimitive> From<&$src<I>>
            for $dst<O>
        {
            fn from(pix: &$src<I>) -> Self {
                Self::from(*pix)
            }
        }
    };
}

macro_rules! impl_from_rgb_to_gray {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: StorageType + AsPrimitive<f32>, O: StorageType + FromPrimitive> From<$src<I>>
            for $dst<O>
        {
            fn from(pix: $src<I>) -> Self {
                // rec601 luma
                let y = O::from_f32(
                    0.2126 * pix[$r].as_() + 0.7152 * pix[$g].as_() + 0.0722 * pix[$b].as_(),
                )
                .unwrap();

                $dst { 0: [y] }
            }
        }

        impl_from_self_ref!($src, $dst);
    };
}

create_pixel!(Gray, 1, #[doc = "Grayscale pixel"]);

impl_from_rgb_to_gray!(Rgb, Gray, 0, 1, 2);
impl_from_rgb_to_gray!(Rgba, Gray, 0, 1, 2);
impl_from_rgb_to_gray!(Bgr, Gray, 2, 1, 0);
impl_from_rgb_to_gray!(Bgra, Gray, 2, 1, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let pix: Gray<u8> = Gray { 0: [255; 1] };

        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn cast_from_slice() {
        let mem = vec![255; 1];
        let pix = Gray::<u8>::cast_from_slice(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn cast_from_slice_mut() {
        let mut mem = vec![255; 1];
        let pix = Gray::<u8>::cast_from_slice_mut(&mut mem).unwrap();

        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn try_from() {
        let mem = vec![255; 1];
        let pix: Gray<u8> = Pixel::try_from(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
    }

    #[test]
    fn channels() {
        assert_eq!(Gray::<u8>::channels(), 1);
    }
}
