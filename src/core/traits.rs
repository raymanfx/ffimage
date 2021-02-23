use std::{mem, ops::IndexMut};

/// Generic pixel container
pub trait Pixel: Sized + Copy + IndexMut<usize> {
    /// Type of the container elements
    type T;

    /// Number of channels for this pixel
    fn channels() -> u8;

    /// Size of one pixel in bytes
    fn len() -> usize {
        Self::channels() as usize * mem::size_of::<Self::T>()
    }
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
pub trait GenericImageView<'a> {
    /// Pixel type
    type T: Pixel;

    /// Type of sub images
    type SubImage;

    /// Width in pixels
    fn width(&self) -> u32;

    /// Height in pixels
    fn height(&self) -> u32;

    /// Returns the pixel at the specified coordinates
    fn pixel(&self, x: u32, y: u32) -> Option<Self::T>;

    /// Returns a sub image view backed by the same data
    fn view(&'a self, x: u32, y: u32, width: u32, height: u32) -> Option<Self::SubImage>;
}

/// Buffered image, provides read-write pixel access
pub trait GenericImage<'a>: GenericImageView<'a> {
    /// Sets the pixel values at the specified coordinates
    fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), ()>;
}

/// Convert between images
pub trait Convert<B> {
    /// Converts the buffer into another, possibly with a different format
    fn convert(&self, output: &mut B);
}
