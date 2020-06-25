use std::convert::TryFrom;
use std::mem;

use crate::core::traits::{Pixel, StorageType};
use crate::packed::image::generic::GenericView;

/// Image view parametrized by its pixel type
pub struct DynamicView<'a, T> {
    pub raw: &'a [T],
    pub width: u32,
    pub height: u32,
    pub stride: usize,
}

impl<'a, T: StorageType> DynamicView<'a, T> {
    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `channels` - Number of channels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::<u8>::new(&mem, 2, 2, 3)
    ///     .expect("Memory region too small");
    /// ```
    pub fn new(raw: &'a [T], width: u32, height: u32, channels: u32) -> Option<Self> {
        let min_stride = width as usize * channels as usize * mem::size_of::<T>();
        let raw_len = raw.len() * mem::size_of::<T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                stride: min_stride,
            })
        }
    }

    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::<u8>::with_stride(&mem, 2, 2, 6)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride(raw: &'a [T], width: u32, height: u32, stride: usize) -> Option<Self> {
        let len = height as usize * stride;
        let raw_len = raw.len() * mem::size_of::<T>();

        if stride > 0 && raw_len != len {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                stride,
            })
        }
    }
}

impl<'a, T> TryFrom<&DynamicView<'a, T::T>> for GenericView<'a, T>
where
    T: Pixel,
{
    type Error = ();

    fn try_from(input: &DynamicView<'a, T::T>) -> Result<Self, Self::Error> {
        let view =
            GenericView::<T>::with_stride(input.raw, input.width, input.height, input.stride);
        match view {
            Some(view) => Ok(view),
            None => Err(()),
        }
    }
}
