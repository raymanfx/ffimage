/// Convert into a slice of types
pub trait ConvertSlice<DP>: Sized {
    /// Converts a slice of pixels into another format
    fn convert_slice<IT: AsRef<[Self]>, OT: AsMut<[DP]>>(input: IT, output: OT);
}
