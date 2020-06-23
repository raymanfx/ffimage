use crate::core::traits::{Convert, ImageView, Pixel, Resize, TryConvert, TryConvertSlice};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_Convert {
    () => {
        fn convert(&self, output: &mut GenericBuffer<DP>) {
            output.resize(self.width(), self.height());

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = self.pixel_row(i).unwrap();
                let row_out = output.pixel_row_mut(i).unwrap();
                row_in.try_convert(row_out).unwrap();
            }
        }
    };
}

macro_rules! impl_TryConvert {
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
                row_in.try_convert(row_out).unwrap();
            }

            Ok(())
        }
    };
}

impl<'a, SP, DP> Convert<GenericBuffer<DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, SP, DP> Convert<GenericBuffer<DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_Convert!();
}

impl<SP, DP> Convert<GenericBuffer<DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericFlatBuffer<'a, DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}
