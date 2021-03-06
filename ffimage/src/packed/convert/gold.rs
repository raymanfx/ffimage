use std::ops::Index;

use crate::traits::{GenericImageView, Pixel, Convert};
use crate::packed::traits::ConvertSlice;
use crate::packed::Image;

impl <DP, I> Convert<Image<DP, &mut [DP::T]>> for I
where
    DP: Pixel + Copy,
    DP::T: Copy,
    I: GenericImageView + Index<usize> + Sync,
    <I as Index<usize>>::Output: Index<usize>,
    <I as Index<usize>>::Output: AsRef<[<<I as Index<usize>>::Output as Index<usize>>::Output]>,
    <<I as Index<usize>>::Output as Index<usize>>::Output: Pixel + ConvertSlice<DP>,

{
    fn convert(&self, output: &mut Image<DP, &mut [DP::T]>) {
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

impl <DP, I> Convert<Image<DP, Vec<DP::T>>> for I
where
    DP: Pixel + Copy,
    DP::T: Copy + Default,
    I: GenericImageView + Index<usize> + Sync,
    <I as Index<usize>>::Output: Index<usize>,
    <I as Index<usize>>::Output: AsRef<[<<I as Index<usize>>::Output as Index<usize>>::Output]>,
    <<I as Index<usize>>::Output as Index<usize>>::Output: Pixel + ConvertSlice<DP>,

{
    fn convert(&self, output: &mut Image<DP, Vec<DP::T>>) {
        if output.width() != self.width() || output.height() != self.height() {
            *output = Image::new(self.width(), self.height(), DP::T::default());
        }

        let row_count = output.height();

        (0..row_count).into_iter().for_each(|i| {
            let row_in = &self[i as usize];
            let row_out = &mut output[i as usize];
            <<Self as Index<usize>>::Output as Index<usize>>::Output::convert(row_in, row_out);
        });
    }
}
