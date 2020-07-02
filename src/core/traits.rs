use std::{array, fmt, mem, ops::IndexMut};

/// Pixel backing type
pub trait StorageType: Default + Copy + Send + Sync {}

impl StorageType for u8 {}
impl StorageType for u16 {}
impl StorageType for u32 {}
impl StorageType for u64 {}

impl StorageType for f32 {}
impl StorageType for f64 {}

/// Generic pixel container
pub trait Pixel: Sized + Default + Copy + Send + Sync + IndexMut<usize> {
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

/// Cloneable images
pub trait CloneImage {
    type Output: ImageBuffer;

    /// Clone an image into an already existing buffer, avoiding reallocations if possible
    fn clone_into(&self, output: &mut Self::Output);

    /// Clone an image type (view or buffer) and return a buffer
    fn clone(&self) -> Self::Output;
}

/// Resizable images
pub trait Resize {
    /// Resize the underlying container to at least fit the number of pixels required
    fn resize(&mut self, width: u32, height: u32);
}

/// Convert between images
pub trait TryConvert<B> {
    type Error: fmt::Debug;

    /// Converts the buffer into another, possibly with a different format
    fn try_convert(&self, output: &mut B) -> Result<(), Self::Error>;
}

/// Convert into a slice of types
pub trait TryConvertSlice<O> {
    type Error: fmt::Debug;

    /// Converts the buffer into another, possibly with a different format
    fn try_convert(&self, output: &mut [O]) -> Result<(), Self::Error>;
}

// Blanket implementation for pixel row conversion.
// If we know how to convert a single pixel into another one, we can automatically convert between
// rows as well. This obviously does not work for macropixels, where one pixel may transform into
// several, so you need to implement the trait yourself for those types.

impl<SP: Pixel, DP: From<SP>> TryConvertSlice<DP> for [SP] {
    type Error = ();

    fn try_convert(&self, output: &mut [DP]) -> Result<(), Self::Error> {
        if self.len() != output.len() {
            return Err(());
        }

        for i in 0..self.len() {
            output[i] = DP::from(self[i]);
        }

        Ok(())
    }
}
