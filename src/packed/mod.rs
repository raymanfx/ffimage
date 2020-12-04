pub mod traits;

pub mod generic;
pub use generic::{
    ImageBuffer as GenericBuffer, ImageView as GenericView, ImageViewMut as GenericViewMut,
};

pub mod dynamic;
pub use dynamic::{ImageBuffer as DynamicBuffer, ImageView as DynamicView};

pub mod convert;
