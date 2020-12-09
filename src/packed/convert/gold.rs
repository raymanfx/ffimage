use std::ops::Index;

use crate::core::traits::{GenericImageView, Pixel, Convert};
use crate::packed::traits::ConvertPixels;
use crate::packed::generic::{ImageBuffer, ImageViewMut};

impl<'a, 'b, DP, I> Convert<ImageViewMut<'b, DP>> for I
where
DP: Pixel,
I: GenericImageView<'a> + Index<usize>,
for <'c> &'c mut <ImageBuffer<DP> as Index<usize>>::Output: IntoIterator,
for <'c> &'c <Self as Index<usize>>::Output: ConvertPixels<&'c mut <ImageBuffer<DP> as Index<usize>>::Output>,
{
    fn convert(&self, output: &mut ImageViewMut<'b, DP>) {
        // how many rows can we convert?
        let row_count = if output.height() > self.height() {
            self.height()
        } else {
            output.height()
        };

        (0..row_count as usize).into_iter().for_each(|i| {
            let input = &self[i];
            let output = &mut output[i];
            input.convert(output);
        });
    }
}

impl<'a, DP, I> Convert<ImageBuffer<DP>> for I
where
DP: Pixel,
I: GenericImageView<'a> + Index<usize>,
for <'b> &'b mut <ImageBuffer<DP> as Index<usize>>::Output: IntoIterator,
for <'b> &'b <Self as Index<usize>>::Output: ConvertPixels<&'b mut <ImageBuffer<DP> as Index<usize>>::Output>,
{
    fn convert(&self, output: &mut ImageBuffer<DP>) {
        if output.width() != self.width() || output.height() != self.height() {
            *output = ImageBuffer::new(self.width(), self.height());
        }

        (0..self.height() as usize).into_iter().for_each(|i| {
            let input = &self[i];
            let output = &mut output[i];
            input.convert(output);
        });
    }
}
