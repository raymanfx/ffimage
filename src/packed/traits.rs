use crate::core::traits::Pixel;

/// Access pixel (read-only)
///
/// This trait guarantees efficient pixel access in a way that the backing pixel storage is already
/// allocated. Packed images allow for reinterpreting raw but coherent memory as a set of pixels.
pub trait AccessPixel {
    type PixelType: Pixel;

    /// Returns the pixel row at the specified y offset
    fn row(&self, y: u32) -> Option<&[Self::PixelType]>;

    /// Returns the pixel at the specified coordinates
    fn get(&self, x: u32, y: u32) -> Option<&Self::PixelType> {
        let row = self.row(y)?;
        let x = x / Self::PixelType::subpixels() as u32;
        if x as usize >= row.len() {
            return None;
        }
        Some(&row[x as usize])
    }
}

/// Access pixel (mutable)
///
/// This trait guarantees efficient pixel access in a way that the backing pixel storage is already
/// allocated. Packed images allow for reinterpreting raw but coherent memory as a set of pixels.
pub trait AccessPixelMut {
    type PixelType: Pixel;

    /// Returns the pixel row at the specified y offset
    fn row_mut(&mut self, y: u32) -> Option<&mut [Self::PixelType]>;

    /// Returns the pixel at the specified coordinates
    fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::PixelType> {
        let row = self.row_mut(y)?;
        let x = x / Self::PixelType::subpixels() as u32;
        if x as usize >= row.len() {
            return None;
        }
        Some(&mut row[x as usize])
    }
}
