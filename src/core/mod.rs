pub mod traits;
pub use traits::{Convert, GenericImage, GenericImageView};
pub use traits::{Macropixel, Pixel};

pub mod pixel;

pub mod iter;
pub use iter::{PixelIter, PixelIterMut};
