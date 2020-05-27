pub mod traits;
pub use traits::{AccessPixel, AccessPixelMut};

pub mod image;
pub use image::{
    GenericBuffer as GenericImageBuffer, GenericFlatBuffer as GenericImageFlatBuffer,
    GenericView as GenericImageView,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        pub mod convert_rayon;
    } else {
        pub mod convert;
    }
}
