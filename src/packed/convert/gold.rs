use crate::core::traits::{GenericImageView, Pixel, Convert, ConvertSlice};
use crate::packed::generic::{ImageBuffer, ImageView, ImageViewMut};

macro_rules! impl_Convert {
    () => {
        fn convert(&self, output: &mut ImageBuffer<DP>) {
            if output.width() != self.width() || output.height() != self.height() {
                *output = ImageBuffer::new(self.width(), self.height());
            }

            let row_count = output.height();

            // iterate over the source pixels and convert them
            for i in 0..row_count {
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                SP::convert(row_in, row_out);
            }
        }
    };
}

macro_rules! impl_ConvertFlat {
    () => {
        fn convert(&self, output: &mut ImageViewMut<DP>) {
            let row_count = if output.height() < self.height() {
                output.height()
            } else {
                self.height()
            };

            // iterate over the source pixels and convert them
            for i in 0..row_count {
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                SP::convert(row_in, row_out);
            }
        }
    };
}

impl<'a, SP, DP> Convert<ImageBuffer<DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, SP, DP> Convert<ImageBuffer<DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<SP, DP> Convert<ImageBuffer<DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, SP, DP> Convert<ImageViewMut<'a, DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}

impl<'a, SP, DP> Convert<ImageViewMut<'a, DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}

impl<'a, SP, DP> Convert<ImageViewMut<'a, DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}
