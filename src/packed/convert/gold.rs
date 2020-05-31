use std::convert::From;

use crate::core::traits::{Convert, ImageBuffer, ImageView, Pixel, Resize, TryConvert};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};

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
                        let src_pix = self.get_pixel(j, i).unwrap();
                        let dst_pix = DP::from(src_pix);
                        output.set_pixel(j, i, &dst_pix).unwrap();
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
                        let src_pix = self.get_pixel(j, i).unwrap();
                        let dst_pix = DP::from(src_pix);
                        output.set_pixel(j, i, &dst_pix).unwrap();
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
