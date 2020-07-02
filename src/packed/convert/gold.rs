use crate::core::traits::{ImageView, Pixel, Resize, TryConvert, TryConvertSlice};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_TryConvert {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericBuffer<DP>) -> Result<(), Self::Error> {
            output.resize(self.width(), self.height());

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = self.pixel_row(i).unwrap();
                let row_out = output.pixel_row_mut(i).unwrap();
                let res = row_in.try_convert(row_out);
                if res.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }
    };
}

macro_rules! impl_TryConvertFlat {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericFlatBuffer<DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                return Err(());
            }

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = self.pixel_row(i).unwrap();
                let row_out = output.pixel_row_mut(i).unwrap();
                let res = row_in.try_convert(row_out);
                if res.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }
    };
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<SP, DP> TryConvert<GenericBuffer<DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}
