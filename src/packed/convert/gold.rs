use crate::core::traits::{Convert, ImageView, Pixel, Resize, TryConvert, TryConvertSlice};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_Convert {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> Convert<$dst<'a, DP>> for $src<'a, SP>
        where
            SP: Pixel,
            DP: Pixel,
            [SP]: TryConvertSlice<DP>,
        {
            fn convert(&self, output: &mut $dst<'a, DP>) {
                output.resize(self.width(), self.height());

                // iterate over the source pixels and convert them
                for i in 0..self.height() {
                    let row_in = self.pixel_row(i).unwrap();
                    let row_out = output.pixel_row_mut(i).unwrap();
                    row_in.try_convert(row_out).unwrap();
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
            DP: Pixel,
            [SP]: TryConvertSlice<DP>,
        {
            type Error = ();

            fn try_convert(&self, output: &mut $dst<'a, DP>) -> Result<(), Self::Error> {
                if output.width() < self.width() || output.height() < self.height() {
                    return Err(());
                }

                // iterate over the source pixels and convert them
                for i in 0..self.height() {
                    let row_in = self.pixel_row(i).unwrap();
                    let row_out = output.pixel_row_mut(i).unwrap();
                    row_in.try_convert(row_out).unwrap();
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
