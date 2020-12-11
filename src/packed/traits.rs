/// Convert into a slice of types
pub trait ConvertSlice<DP>: Sized {
    /// Converts the buffer into another, possibly with a different format
    fn convert<IT: AsRef<[Self]>, OT: AsMut<[DP]>>(input: IT, output: OT);
}
