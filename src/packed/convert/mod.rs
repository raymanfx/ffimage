use std::convert::From;

use crate::core::traits::{Convert, ImageView, Pixel, TryConvertSlice};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};

cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        pub mod rayon;
    } else {
        pub mod gold;
    }
}

macro_rules! impl_From {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> From<&$src<'a, SP>> for $dst<'a, DP>
        where
            SP: Pixel,
            DP: Pixel,
            [SP]: TryConvertSlice<DP>,
        {
            fn from(input: &$src<'a, SP>) -> Self {
                let mut output = Self::new(input.width(), input.height());
                input.convert(&mut output);
                output
            }
        }
    };
}

impl_From!(GenericView, GenericBuffer);
impl_From!(GenericFlatBuffer, GenericBuffer);
impl_From!(GenericBuffer, GenericBuffer);
