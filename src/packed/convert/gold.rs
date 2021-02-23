use std::ops::Index;

use num_traits::identities::Zero;

use crate::core::traits::{GenericImageView, Pixel, Convert};
use crate::packed::traits::ConvertSlice;
use crate::packed::generic::{ImageBuffer, ImageViewMut};

impl <'a, 'b, DP, I> Convert<ImageViewMut<'b, DP>> for I
where
    DP: Pixel,
    DP::T: Copy + Zero,
    I: GenericImageView<'a> + Index<usize> + Sync,
    <I as Index<usize>>::Output: Index<usize>,
    <I as Index<usize>>::Output: AsRef<[<<I as Index<usize>>::Output as Index<usize>>::Output]>,
    <<I as Index<usize>>::Output as Index<usize>>::Output: Pixel + ConvertSlice<DP>,

{
    fn convert(&self, output: &mut ImageViewMut<'b, DP>) {
        let row_count = if output.height() < self.height() {
            output.height()
        } else {
            self.height()
        };

        (0..row_count).into_iter().for_each(|i| {
            let row_in = &self[i as usize];
            let row_out = &mut output[i as usize];
            <<Self as Index<usize>>::Output as Index<usize>>::Output::convert(row_in, row_out);
        });
    }
}

impl <'a, DP, I> Convert<ImageBuffer<DP>> for I
where
    DP: Pixel,
    DP::T: Copy + Zero,
    I: GenericImageView<'a> + Index<usize> + Sync,
    <I as Index<usize>>::Output: Index<usize>,
    <I as Index<usize>>::Output: AsRef<[<<I as Index<usize>>::Output as Index<usize>>::Output]>,
    <<I as Index<usize>>::Output as Index<usize>>::Output: Pixel + ConvertSlice<DP>,

{
    fn convert(&self, output: &mut ImageBuffer<DP>) {
        if output.width() != self.width() || output.height() != self.height() {
            *output = ImageBuffer::new(self.width(), self.height());
        }

        let row_count = output.height();

        (0..row_count).into_iter().for_each(|i| {
            let row_in = &self[i as usize];
            let row_out = &mut output[i as usize];
            <<Self as Index<usize>>::Output as Index<usize>>::Output::convert(row_in, row_out);
        });
    }
}
