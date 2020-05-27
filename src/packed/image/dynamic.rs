use std::convert::TryFrom;
use std::mem;

use crate::color::*;
use crate::core::traits::StorageType;
use crate::packed::image::generic::GenericView;

/// Describes an image format in terms of color scheme and number of channels.
///
/// The color string should indicate the underlying color model of the image, e.g. "Gray" or "Rgb".
/// It is used in the conversion methods to cast the raw bytes to a strongly typed image view.
/// The number of channels are used to derive the bytes per pixel and thus validate the raw buffer
/// buffer length.
pub struct FormatHint {
    /// Color model representation, e.g. "RGB"
    pub color: String,
    /// Number of channels
    pub channels: usize,
}

/// Image view parametrized by its pixel type
pub struct DynamicView<'a, T> {
    pub raw: &'a [T],
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub hint: FormatHint,
}

impl<'a, T: StorageType> DynamicView<'a, T> {
    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `hint` - Format hint
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::{DynamicImageView, FormatHint};
    ///
    /// let mem = vec![0; 14];
    /// let hint = FormatHint { color: String::from("RGB"), channels: 3 };
    /// let view = DynamicImageView::<u8>::new(&mem, 2, 2, hint)
    ///     .expect("Memory region too small");
    /// ```
    pub fn new(raw: &'a [T], width: u32, height: u32, hint: FormatHint) -> Option<Self> {
        let min_stride = width as usize * hint.channels as usize * mem::size_of::<T>();
        let raw_len = raw.len() * mem::size_of::<T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                stride: min_stride,
                hint,
            })
        }
    }

    /// Returns an image view with unknown pixel type
    ///
    /// This constructor takes an additional stride for strided image buffers.
    /// The stride must be a multiple of the size of the internal backing type T of the pixel.
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `hint` - Format hint
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::{DynamicImageView, FormatHint};
    ///
    /// let mem = vec![0; 14];
    /// let hint = FormatHint { color: String::from("RGB"), channels: 3 };
    /// let view = DynamicImageView::<u8>::with_stride(&mem, 2, 2, hint, 7 /* one byte padding */)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride(
        raw: &'a [T],
        width: u32,
        height: u32,
        hint: FormatHint,
        stride: usize,
    ) -> Option<Self> {
        let min_stride = width as usize * hint.channels as usize * mem::size_of::<T>();
        let raw_len = raw.len() * mem::size_of::<T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(DynamicView {
                raw,
                width,
                height,
                stride,
                hint,
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
                match input.hint.color.as_str() {
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
