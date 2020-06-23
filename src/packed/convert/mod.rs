use std::convert::From;

use crate::core::traits::{Convert, ImageView, Pixel};
use crate::packed::image::GenericBuffer;

cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        pub mod rayon;
    } else {
        pub mod gold;
    }
}

impl<'a, S, DP> From<&S> for GenericBuffer<DP>
where
    S: ImageView + Convert<GenericBuffer<DP>>,
    DP: Pixel,
{
    fn from(input: &S) -> Self {
        let mut output = Self::new(input.width(), input.height());
        input.convert(&mut output);
        output
    }
}
