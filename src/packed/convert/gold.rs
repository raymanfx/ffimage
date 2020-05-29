use std::convert::From;

use crate::core::traits::{Convert, ImageView, Pixel, Resize, TryConvert};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_Convert {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> Convert<$dst<'a, DP>> for $src<'a, SP>
        where
            SP: Pixel,
            DP: Pixel + From<SP>,
        {
            fn convert(&self, output: &mut $dst<'a, DP>) {
                output.resize(self.width(), self.height());

                // iterate over the source pixels and convert them
                for i in 0..self.height() {
                    for j in 0..self.width() {
                        let src_pix = self.pixel(j, i).unwrap();
                        *output.pixel_mut(j, i).unwrap() = DP::from(*src_pix);
                    }
                }
            }
        }
    };
}

macro_rules! impl_TryConvert {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> TryConvert<$dst<'a, DP>> for $src<'a, SP>
        where
            SP: Pixel,
            DP: Pixel + From<SP>,
        {
            type Error = ();

            fn try_convert(&self, output: &mut $dst<'a, DP>) -> Result<(), Self::Error> {
                if output.width() < self.width() || output.height() < self.height() {
                    return Err(());
                }

                // iterate over the source pixels and convert them
                for i in 0..self.height() {
                    for j in 0..self.width() {
                        let src_pix = self.pixel(j, i).unwrap();
                        *output.pixel_mut(j, i).unwrap() = DP::from(*src_pix);
                    }
                }

                Ok(())
            }
        }
    };
}

impl_Convert!(GenericView, GenericBuffer);
impl_Convert!(GenericFlatBuffer, GenericBuffer);
impl_Convert!(GenericBuffer, GenericBuffer);

impl_TryConvert!(GenericView, GenericFlatBuffer);
impl_TryConvert!(GenericFlatBuffer, GenericFlatBuffer);
impl_TryConvert!(GenericBuffer, GenericFlatBuffer);
