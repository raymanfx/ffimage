use crate::core::traits::Pixel;

/// Convert into a slice of types
pub trait ConvertSlice<DP: Pixel>: Sized {
    /// Converts the buffer into another, possibly with a different format
    fn convert(input: &[Self], output: &mut [DP]);
}
