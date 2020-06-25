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
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * channels as usize * mem::size_of::<T>();
        let stride = raw.len() * mem::size_of::<T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(DynamicView {
            raw,
            width,
            height,
            stride,
        })
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
            return None;
        }

        Some(DynamicView {
            raw,
            width,
            height,
            stride,
        })
    }
}

impl<'a, T> TryFrom<&DynamicView<'a, T::T>> for GenericView<'a, T>
where
    T: Pixel,
{
    type Error = ();

    fn try_from(input: &DynamicView<'a, T::T>) -> Result<Self, Self::Error> {
        let view = GenericView::<T>::new(input.raw, input.width, input.height);
        match view {
            Some(view) => Ok(view),
            None => Err(()),
        }
    }
}

/// Image buffer parametrized by its pixel type
pub struct DynamicBuffer<T> {
    pub raw: Vec<T>,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
}

impl<T: StorageType> DynamicBuffer<T> {
    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `channels` - Number of channels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageBuffer;
    ///
    /// let buf = DynamicImageBuffer::<u8>::new(2, 2, 3);
    /// ```
    pub fn new(width: u32, height: u32, channels: u32) -> Self {
        let stride = width as usize * channels as usize * mem::size_of::<T>();
        let mut buf = DynamicBuffer {
            raw: Vec::new(),
            width,
            height,
            stride,
        };

        buf.raw
            .resize((height * width * channels) as usize, T::default());
        buf
    }

    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `channels` - Number of channels
    /// * `raw` - Raw memory region to interpret as typed image
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageBuffer;
    ///
    /// let mem = vec![0; 12];
    /// let buf = DynamicImageBuffer::<u8>::with_raw(2, 2, 3, &mem);
    /// ```
    pub fn with_raw(width: u32, height: u32, channels: u32, raw: &[T]) -> Self {
        let stride = width as usize * channels as usize * mem::size_of::<T>();
        DynamicBuffer {
            raw: raw.to_vec(),
            width,
            height,
            stride,
        }
    }

    pub fn raw(&self) -> &[T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T] {
        &mut self.raw
    }

    pub fn resize(&mut self, width: u32, height: u32, channels: u32) {
        self.width = width;
        self.height = height;
        self.stride = width as usize * channels as usize * mem::size_of::<T>();
        self.raw
            .resize((height * width * channels) as usize, T::default());
    }
}
