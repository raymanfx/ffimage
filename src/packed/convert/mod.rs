use crate::packed::traits::ConvertPixels;

// Blanket implementation for pixel row conversion.
// If we know how to convert a single pixel into another one, we can automatically convert between
// rows as well. This obviously does not work for macropixels, where one pixel may transform into
// several, so you need to implement the trait yourself for those types.

impl<'a, 'b, T1: 'a, T2: 'b, II, OI> ConvertPixels<OI> for II
where
    T1: Copy,
    T2: From<T1>,
    II: IntoIterator<Item = &'a T1>,
    OI: IntoIterator<Item = &'b mut T2>,
{
    fn convert(self, output: OI) {
        let values = self.into_iter().zip(output.into_iter());
        for (val_in, val_out) in values {
            *val_out = T2::from(*val_in);
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
