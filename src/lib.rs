pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::{PixelIter, PixelIterMut},
        core::traits::{Convert, ImageBuffer, ImageView, Pixel, Resize, TryConvert},
        packed::traits::{AccessPixel, AccessPixelMut},
        packed::{GenericImageBuffer, GenericImageFlatBuffer, GenericImageView},
    };
}
