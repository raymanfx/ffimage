use crate::core::traits::{ImageBuffer, ImageView};

/// An iterator type for images to iterate through pixels
///
/// The actual item type (e.g. full object or reference) depends on the implementation.
/// This allows for maximum performance for types which may just reinterpret memory regions as
/// pixels instead of creating them anew for each coordinate pair.
pub struct PixelIter<'a, I> {
    pub img: &'a I,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl<'a, I: ImageView> PixelIter<'a, I> {
    /// Returns an iterator which goes through all image pixel rows
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    pub fn new(img: &'a I) -> Self {
        PixelIter {
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
        PixelIter {
            img,
            x,
            y,
            width: img.width(),
            height: img.height(),
        }
    }
}

/// An iterator type for images to iterate through pixels and mutate them
///
/// The actual item type (e.g. full object or reference) depends on the implementation.
/// This allows for maximum performance for types which may just reinterpret memory regions as
/// pixels instead of creating them anew for each coordinate pair.
pub struct PixelIterMut<'a, I> {
    pub img: &'a mut I,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl<'a, I: ImageBuffer> PixelIterMut<'a, I> {
    /// Returns an iterator which goes through all image pixel rows
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    pub fn new(img: &'a mut I) -> Self {
        PixelIterMut {
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
    pub fn with_offset(img: &'a mut I, x: u32, y: u32) -> Self {
        PixelIterMut {
            img,
            x,
            y,
            width: img.width(),
            height: img.height(),
        }
    }
}
