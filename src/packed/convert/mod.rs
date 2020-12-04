use crate::core::traits::Pixel;
use crate::packed::traits::ConvertSlice;

// Blanket implementation for pixel row conversion.
// If we know how to convert a single pixel into another one, we can automatically convert between
// rows as well. This obviously does not work for macropixels, where one pixel may transform into
// several, so you need to implement the trait yourself for those types.

impl<SP: Pixel, DP: Pixel + From<SP>> ConvertSlice<DP> for SP {
    fn convert(input: &[SP], output: &mut [DP]) {
        let pixels = input.iter().zip(output.iter_mut());
        for (pix_in, pix_out) in pixels {
            *pix_out = DP::from(*pix_in);
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        pub mod rayon;
    } else {
        pub mod gold;
    }
}
