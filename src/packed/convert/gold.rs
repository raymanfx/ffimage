use crate::core::traits::{GenericImageView, Pixel, TryConvert, TryConvertSlice};
use crate::packed::generic::{ImageBuffer, ImageView, ImageViewMut};

macro_rules! impl_TryConvert {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut ImageBuffer<DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                *output = ImageBuffer::new(self.width(), self.height());
            }

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                let res = SP::try_convert(row_in, row_out);
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

        fn try_convert(&self, output: &mut ImageViewMut<DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                return Err(());
            }

            // iterate over the source pixels and convert them
            for i in 0..self.height() {
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                let res = SP::try_convert(row_in, row_out);
                if res.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }
    };
}

impl<'a, SP, DP> TryConvert<ImageBuffer<DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<ImageBuffer<DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<SP, DP> TryConvert<ImageBuffer<DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<ImageViewMut<'a, DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<ImageViewMut<'a, DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, SP, DP> TryConvert<ImageViewMut<'a, DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}
