pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::{PixelIter, PixelIterMut},
        core::traits::{Convert, ImageBuffer, ImageView, Pixel, Resize, TryConvert},
        packed::image::{
            DynamicView as DynamicImageView, GenericBuffer as GenericImageBuffer,
            GenericFlatBuffer as GenericImageFlatBuffer, GenericView as GenericImageView,
        },
        packed::traits::{AccessPixel, AccessPixelMut},
    };
}
