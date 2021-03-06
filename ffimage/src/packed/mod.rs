pub mod traits;
pub use traits::ConvertSlice;

pub mod matrix;
pub use matrix::Matrix;

pub mod image;
pub use image::Image;

pub mod convert;
pub mod iter;

use crate::traits::Pixel;

pub type ImageView<'a, T> = Image<T, &'a [<T as Pixel>::T]>;
pub type ImageViewMut<'a, T> = Image<T, &'a mut [<T as Pixel>::T]>;
pub type ImageBuffer<'a, T> = Image<T, Vec<<T as Pixel>::T>>;
