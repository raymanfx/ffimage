use ffimage::packed::traits::ConvertSlice;
use ffimage::traits::Pixel;
use ffimage::{create_macropixel, define_pixel, impl_Pixel};

use crate::yuv::*;

create_macropixel!(Yuyv, 4, 2, #[doc = "Yuyv macropixel"]);

impl<T: Copy> From<Yuyv<T>> for [Yuv<T>; 2] {
    fn from(pix: Yuyv<T>) -> Self {
        let _1 = Yuv {
            0: [pix[0], pix[1], pix[3]],
        };
        let _2 = Yuv {
            0: [pix[2], pix[1], pix[3]],
        };

        [_1, _2]
    }
}

impl<T: Copy> From<[Yuv<T>; 2]> for Yuyv<T> {
    fn from(pix: [Yuv<T>; 2]) -> Self {
        Yuyv {
            0: [pix[0][0], pix[0][1], pix[1][0], pix[0][2]],
        }
    }
}

impl<T: Copy> ConvertSlice<Yuv<T>> for Yuyv<T> {
    fn convert<IT: AsRef<[Self]>, OT: AsMut<[Yuv<T>]>>(input: IT, mut output: OT) {
        for (outp, inp) in output
            .as_mut()
            .chunks_exact_mut(2)
            .zip(input.as_ref().iter())
        {
            let yuv = <[Yuv<T>; 2]>::from(*inp);
            outp[0] = yuv[0];
            outp[1] = yuv[1];
        }
    }
}

impl<T: Copy> ConvertSlice<Yuyv<T>> for Yuv<T> {
    fn convert<IT: AsRef<[Self]>, OT: AsMut<[Yuyv<T>]>>(input: IT, mut output: OT) {
        for (outp, inp) in output
            .as_mut()
            .iter_mut()
            .zip(input.as_ref().chunks_exact(2))
        {
            let yuyv = Yuyv::<T>::from([inp[0], inp[1]]);
            *outp = yuyv;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        assert_eq!(Yuyv::<u8>::channels(), 4);
    }
}
