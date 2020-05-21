use crate::core::traits::ImageView;

/// An iterator type for images to iterate through pixels
///
/// The actual item type (e.g. full object or reference) depends on the implementation.
/// This allows for maximum performance for types which may just reinterpret memory regions as
/// pixels instead of creating them anew for each coordinate pair.
pub struct PixelIterator<'a, I> {
    pub img: &'a I,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl<'a, I: ImageView> PixelIterator<'a, I> {
    /// Returns an iterator which goes through all image pixel rows
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    pub fn new(img: &'a I) -> Self {
        PixelIterator {
            img,
            x: 0,
            y: 0,
            width: img.width(),
            height: img.height(),
        }
    }

    /// Returns an iterator which starts at the specified coordinate offsets
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    /// * `x` - X offset
    /// * `y` - Y offset
    pub fn with_offset(img: &'a I, x: u32, y: u32) -> Self {
        PixelIterator {
            img,
            x,
            y,
            width: img.width(),
            height: img.height(),
        }
    }
}
