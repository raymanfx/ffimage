use num_traits::{AsPrimitive, FromPrimitive};

use ffimage::color::bgr::*;
use ffimage::color::rgb::*;
use ffimage::traits::Pixel;
use ffimage::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_rgb_to_yuv {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: AsPrimitive<i32>, O: FromPrimitive> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                let r = pix[$r].as_();
                let g = pix[$g].as_();
                let b = pix[$b].as_();

                let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
                let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
                let v = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;

                let y = O::from_i32(y).unwrap();
                let u = O::from_i32(u).unwrap();
                let v = O::from_i32(v).unwrap();
                $dst { 0: [y, u, v] }
            }
        }
    };
}

macro_rules! impl_from_yuv_to_rgb {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: AsPrimitive<i32>, O: Copy + FromPrimitive> From<$src<I>> for $dst<O> {
            fn from(pix: $src<I>) -> Self {
                let y = pix[0].as_();
                let u = pix[1].as_();
                let v = pix[2].as_();
                let c = y - 16;
                let d = u - 128;
                let e = v - 128;

                let r = num_traits::clamp((298 * c + 409 * e + 128) >> 8, 0, 255);
                let g = num_traits::clamp((298 * c - 100 * d - 208 * e + 128) >> 8, 0, 255);
                let b = num_traits::clamp((298 * c + 516 * d + 128) >> 8, 0, 255);

                let r = O::from_i32(r).unwrap();
                let g = O::from_i32(g).unwrap();
                let b = O::from_i32(b).unwrap();

                let mut result = $dst { 0: [r, g, b] };
                result[$r] = r;
                result[$g] = g;
                result[$b] = b;
                result
            }
        }
    };
}

create_pixel!(Yuv, 3, #[doc = "YUV pixel"]);

impl_from_rgb_to_yuv!(Bgr, Yuv, 2, 1, 0);
impl_from_rgb_to_yuv!(Rgb, Yuv, 0, 1, 2);

impl_from_yuv_to_rgb!(Yuv, Bgr, 2, 1, 0);
impl_from_yuv_to_rgb!(Yuv, Rgb, 0, 1, 2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Yuv::<u8>::channels(), 3);
    }
}
