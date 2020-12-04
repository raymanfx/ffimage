pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::{PixelIter, PixelIterMut},
        core::traits::{Convert, GenericImage, GenericImageView, Macropixel, Pixel},
        packed::dynamic,
        packed::generic,
        packed::generic::{
            ImageBuffer as PackedImageBuffer, ImageView as PackedImageView,
            ImageViewMut as PackedImageViewMut,
        },
    };
}
