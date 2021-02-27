pub mod traits;

pub mod generic;
pub use generic::Image;

pub mod iter;

pub type ImageView<'a, T> = Image<T, &'a [T]>;
pub type ImageViewMut<'a, T> = Image<T, &'a mut [T]>;
pub type ImageBuffer<'a, T> = Image<T, Vec<T>>;

pub mod dynamic;
pub use dynamic::{ImageBuffer as DynamicBuffer, ImageView as DynamicView};

pub mod convert;
