use std::{array, fmt, mem, ops::IndexMut};

use num_traits::Num;

/// Pixel backing type
pub trait StorageType: Num + Copy + Send + Sync {}

impl<T: Num + Copy + Send + Sync> StorageType for T {}

/// Generic pixel container
pub trait Pixel: Sized + Copy + Send + Sync + IndexMut<usize> {
    /// Type of the container elements
    type T: StorageType;

    /// Returns the channel value at the specified index
    fn at(&self, index: usize) -> Self::T;

    /// Convert a memory region into a pixel by copying the bytes
    fn try_from(raw: &[Self::T]) -> Result<Self, array::TryFromSliceError>;

    /// Number of channels for this pixel
    fn channels() -> u8;

    /// Size of one pixel in bytes
    fn len() -> usize {
        Self::channels() as usize * mem::size_of::<Self::T>()
    }

    /// Number of image pixels for this pixel
    fn subpixels() -> u8;
}

/// Macropixel container
pub trait Macropixel: Pixel {
    /// Type of the image pixel
    type Subpixel: Pixel;

    /// Convert image pixels into a macropixel
    fn from_subpixels(pixels: &[Self::Subpixel]) -> Self;

    /// Convert into image pixels
    fn to_subpixels(&self) -> [Self::Subpixel];
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

/// Convert between images
pub trait TryConvert<B> {
    type Error: fmt::Debug;

    /// Converts the buffer into another, possibly with a different format
    fn try_convert(&self, output: &mut B) -> Result<(), Self::Error>;
}

/// Convert into a slice of types
pub trait TryConvertSlice<DP: Pixel>: Sized {
    type Error: fmt::Debug;

    /// Converts the buffer into another, possibly with a different format
    fn try_convert(input: &[Self], output: &mut [DP]) -> Result<(), Self::Error>;
}
