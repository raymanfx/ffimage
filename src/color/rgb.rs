use num::traits::Bounded;
use num_traits::AsPrimitive;

use crate::color::bgr::*;
use crate::color::gray::*;
use crate::core::traits::Pixel;
use crate::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_pix_to_pix3 {
    ($src:ident, $dst:ident, $_0:expr, $_1:expr, $_2:expr) => {
        impl<I: AsPrimitive<O>, O: Copy + 'static> From<$src<I>> for $dst<O> {
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
        impl<I: AsPrimitive<O>, O: Copy + Bounded + 'static> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                $dst {
                    0: [
                        pix[$_0].as_(),
                        pix[$_1].as_(),
                        pix[$_2].as_(),
                        O::max_value(),
                    ],
                }
            }
        }
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
    fn channels() {
        assert_eq!(Rgb::<u8>::channels(), 3);
        assert_eq!(Rgba::<u8>::channels(), 4);
    }

    #[test]
    fn index_mut() {
        let pix: Rgb<u8> = Rgb { 0: [1, 2, 3] };

        assert_eq!(pix[0], 1);
        assert_eq!(pix[1], 2);
        assert_eq!(pix[2], 3);
    }
}
