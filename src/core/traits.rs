use std::{array, mem, ops::IndexMut};

/// Pixel backing type
pub trait StorageType: Default + Copy + Clone + Send + Sync {}

impl StorageType for u8 {}
impl StorageType for u16 {}
impl StorageType for u32 {}
impl StorageType for u64 {}

impl StorageType for f32 {}
impl StorageType for f64 {}

/// Generic pixel container
pub trait Pixel: Sized + Default + Copy + Clone + Send + Sync + IndexMut<usize> {
    /// Type of the container elements
    type T: StorageType;

    /// Returns the channel value at the specified index
    fn at(&self, index: usize) -> Self::T;

    /// Transmute the slice into a pixel reference
    fn cast_from_slice(raw: &[Self::T]) -> Option<&Self>;

    /// Transmute the slice into a writable pixel reference
    fn cast_from_slice_mut(raw: &mut [Self::T]) -> Option<&mut Self>;

    /// Convert a memory region into a pixel by copying the bytes
    fn try_from(raw: &[Self::T]) -> Result<Self, array::TryFromSliceError>;

    /// Number of channels for this pixel
    fn channels() -> u8;

    /// Size of one pixel in bytes
    fn len() -> usize {
        Self::channels() as usize * mem::size_of::<Self::T>()
    }
}

/// View into an image, provides read-only pixel access
pub trait ImageView {
    /// Pixel type
    type T: Pixel;

    /// Width in pixels
    fn width(&self) -> u32;

    /// Height in pixels
    fn height(&self) -> u32;

    /// Length of one pixel row in bytes
    fn stride(&self) -> usize;

    /// Returns the pixel at the specified coordinates
    fn get_pixel(&self, x: u32, y: u32) -> Option<Self::T>;
}

/// Buffered image, provides read-write pixel access
pub trait ImageBuffer: ImageView {
    /// Sets the pixel values at the specified coordinates
    fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), ()>;
}

/// Resizable images
pub trait Resize {
    /// Resize the underlying container to at least fit the number of pixels required
    fn resize(&mut self, width: u32, height: u32);
}

/// Convert between images
pub trait Convert<B> {
    /// Converts the buffer into another, possibly with a different format
    fn convert(&self, output: &mut B);
}

/// Convert between images
pub trait TryConvert<B> {
    type Error;

    /// Converts the buffer into another, possibly with a different format
    fn try_convert(&self, output: &mut B) -> Result<(), Self::Error>;
}

// For resizable buffers, one should implement the Convert trait because the conversion cannot
// fail. We provide a blanket implementation of the TryConvert trait based on the Convert
// implementation.

impl<B: ImageBuffer + Resize, V: ImageView + Convert<B>> TryConvert<B> for V {
    type Error = ();

    fn try_convert(&self, output: &mut B) -> Result<(), Self::Error> {
        self.convert(output);
        Ok(())
    }
}
