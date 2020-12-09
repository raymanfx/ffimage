/// Access slices
pub(super) trait Slice {
    /// Slice item type
    type Item;

    /// Returns the pixel row at the specified index
    fn slice(&self, index: usize) -> &[Self::Item];
}

/// Access slices
pub(super) trait SliceMut {
    /// Slice item type
    type Item;

    /// Returns the pixel row at the specified index
    fn slice_mut(&self, index: usize) -> &mut [Self::Item];
}

/// Convert into an output iterator
pub trait ConvertPixels<T: IntoIterator + ?Sized> {
    /// Converts the buffer into another, possibly with a different format
    fn convert(self, output: T);
}
