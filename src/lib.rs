pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::{PixelIter, PixelIterMut},
        core::traits::{ImageBuffer, ImageView, Macropixel, Pixel, TryConvert},
        packed::image::{
            DynamicBuffer as DynamicImageBuffer, DynamicStorageType,
            DynamicView as DynamicImageView, GenericBuffer as GenericImageBuffer,
            GenericFlatBuffer as GenericImageFlatBuffer, GenericView as GenericImageView,
        },
        packed::traits::{AccessPixel, AccessPixelMut},
    };
}
