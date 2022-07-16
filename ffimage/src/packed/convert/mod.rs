use crate::packed::traits::ConvertSlice;

// Blanket implementation for pixel row conversion.
// If we know how to convert a single pixel into another one, we can automatically convert between
// rows as well. This obviously does not work for macropixels, where one pixel may transform into
// several, so you need to implement the trait yourself for those types.

impl<SP, DP> ConvertSlice<DP> for SP
where
    SP: Copy,
    DP: From<SP>,
{
    fn convert_slice<IT: AsRef<[Self]>, OT: AsMut<[DP]>>(input: IT, mut output: OT) {
        let pixels = input.as_ref().iter().zip(output.as_mut().iter_mut());
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
