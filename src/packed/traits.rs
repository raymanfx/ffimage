use crate::core::traits::Pixel;

/// Access pixel (read-only)
///
/// This trait guarantees efficient pixel access in a way that the backing pixel storage is already
/// allocated. Packed images allow for reinterpreting raw but coherent memory as a set of pixels.
pub trait AccessPixel {
    type PixelType: Pixel;

    /// Returns the pixel row at the specified y offset
    fn pixel_row(&self, y: u32) -> Option<&[Self::PixelType]>;

    /// Returns the pixel at the specified coordinates
    fn pixel(&self, x: u32, y: u32) -> Option<&Self::PixelType>;
}

/// Access pixel (mutable)
///
/// This trait guarantees efficient pixel access in a way that the backing pixel storage is already
/// allocated. Packed images allow for reinterpreting raw but coherent memory as a set of pixels.
pub trait AccessPixelMut {
    type PixelType: Pixel;

    /// Returns the pixel row at the specified y offset
    fn pixel_row_mut(&mut self, y: u32) -> Option<&mut [Self::PixelType]>;

    /// Returns the pixel at the specified coordinates
    fn pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::PixelType>;
}
