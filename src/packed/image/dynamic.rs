use std::convert::TryFrom;
use std::mem;

use crate::color::*;
use crate::core::traits::StorageType;
use crate::packed::image::generic::GenericView;

/// Image view parametrized by its pixel type
pub struct DynamicView<'a, T> {
    pub raw: &'a [T],
    pub width: u32,
    pub height: u32,
    pub pixfmt: String,
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
    /// * `pixfmt` - Pixelformat
    /// * `channels` - Number of channels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::<u8>::new(&mem, 2, 2, "Rgb", 3)
    ///     .expect("Memory region too small");
    /// ```
    pub fn new(raw: &'a [T], width: u32, height: u32, pixfmt: &str, channels: u32) -> Option<Self> {
        let min_stride = width as usize * channels as usize * mem::size_of::<T>();
        let raw_len = raw.len() * mem::size_of::<T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                pixfmt: String::from(pixfmt),
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
    /// * `pixfmt` - Pixelformat
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::<u8>::with_stride(&mem, 2, 2, "Rgb", 6)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride(
        raw: &'a [T],
        width: u32,
        height: u32,
        pixfmt: &str,
        stride: usize,
    ) -> Option<Self> {
        let len = height as usize * stride;
        let raw_len = raw.len() * mem::size_of::<T>();

        if raw_len != len {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                pixfmt: String::from(pixfmt),
                stride,
            })
        }
    }
}

macro_rules! impl_TryFrom {
    ($pix:ident, $hint:expr) => {
        impl<'a, T> TryFrom<&DynamicView<'a, T>> for GenericView<'a, $pix<T>>
        where
            T: StorageType,
        {
            type Error = ();

            fn try_from(input: &DynamicView<'a, T>) -> Result<Self, Self::Error> {
                match input.pixfmt.as_str() {
                    $hint => {
                        let view = GenericView::<$pix<T>>::with_stride(
                            input.raw,
                            input.width,
                            input.height,
                            input.stride,
                        );
                        match view {
                            Some(view) => Ok(view),
                            None => Err(()),
                        }
                    }
                    _ => Err(()),
                }
            }
        }
    };
}

impl_TryFrom!(Gray, "Gray");
impl_TryFrom!(Rgb, "Rgb");
impl_TryFrom!(Rgba, "Rgba");
impl_TryFrom!(Bgr, "Bgr");
impl_TryFrom!(Bgra, "Bgra");
