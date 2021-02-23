pub mod core;

pub mod color;
pub mod packed;

pub mod prelude {
    pub use crate::{
        color::*,
        core::iter::PixelIter,
        core::traits::{Convert, GenericImage, GenericImageView, Macropixel, Pixel},
        packed::dynamic,
        packed::generic,
        packed::{
            Image as PackedImage, ImageBuffer as PackedImageBuffer, ImageView as PackedImageView,
        },
    };
}
