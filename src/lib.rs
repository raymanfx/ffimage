pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::{PixelIter, PixelIterMut},
        core::traits::{ImageBuffer, ImageView, Macropixel, Pixel, TryConvert},
        packed::dynamic::{
            DynamicBuffer as DynamicImageBuffer, DynamicView as DynamicImageView,
            StorageType as DynamicStorageType,
        },
        packed::generic::{
            GenericBuffer as GenericImageBuffer, GenericFlatBuffer as GenericImageFlatBuffer,
            GenericView as GenericImageView,
        },
    };
}
