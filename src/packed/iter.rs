use std::ops::Index;

use crate::core::traits::GenericImageView;

#[derive(Debug, Clone, Copy)]
/// An iterator type for images to iterate through pixels
///
/// The actual item type (e.g. full object or reference) depends on the implementation.
/// This allows for maximum performance for types which may just reinterpret memory regions as
/// pixels instead of creating them anew for each coordinate pair.
pub struct SliceIter<'a, I> {
    pub img: &'a I,
    pub y: u32,
    pub height: u32,
}

impl<'a, I: GenericImageView<'a>> SliceIter<'a, I> {
    /// Returns an iterator which goes through all image pixel rows
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    pub fn new(img: &'a I) -> Self {
        SliceIter {
            img,
            y: 0,
            height: img.height(),
        }
    }

    /// Returns an iterator which starts at the specified coordinate offsets
    ///
    /// # Arguments
    ///
    /// * `img` - An image instance
    /// * `y` - Y offset
    pub fn with_offset(img: &'a I, y: u32) -> Self {
        SliceIter {
            img,
            y,
            height: img.height(),
        }
    }
}

impl<'a, I: GenericImageView<'a> + Index<usize>> Iterator for SliceIter<'a, I>
where
    <I as Index<usize>>::Output: Sized + 'static,
{
    type Item = &'a <I as Index<usize>>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        Some(self.img.index(self.y as usize))
    }
}
