use num_traits::{AsPrimitive, FromPrimitive};

use crate::color::bgr::*;
use crate::color::rgb::*;
use crate::traits::Pixel;
use crate::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_rgb_to_gray {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: AsPrimitive<f32>, O: FromPrimitive> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                // rec601 luma
                let y = O::from_f32(
                    0.2126 * pix[$r].as_() + 0.7152 * pix[$g].as_() + 0.0722 * pix[$b].as_(),
                )
                .unwrap();

                $dst { 0: [y] }
            }
        }
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
    fn channels() {
        assert_eq!(Gray::<u8>::channels(), 1);
    }

    #[test]
    fn index_mut() {
        let pix: Gray<u8> = Gray { 0: [1] };

        assert_eq!(pix[0], 1);
    }
}
