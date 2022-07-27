use crate::convert::{Convert, MapPixels};
use crate::packed::Image;
use crate::traits::Pixel;

fn _convert<SP, DP, T, U>(input: &Image<SP, T>, output: &mut Image<DP, U>)
where
    SP: Pixel + Copy + MapPixels<SP, DP>,
    DP: Pixel + Copy,
    T: AsRef<[SP::T]>,
    U: AsRef<[DP::T]> + AsMut<[DP::T]>,
{
    input
        .rows()
        .zip(output.rows_mut())
        .for_each(|(in_row, out_row)| SP::map_pixels(in_row, out_row));
}

impl<SP, DP, T, U> Convert<Image<DP, U>> for Image<SP, T>
where
    SP: Pixel + Copy + MapPixels<SP, DP>,
    DP: Pixel + Copy,
    T: AsRef<[SP::T]>,
    U: AsRef<[DP::T]> + AsMut<[DP::T]>,
{
    fn convert(&self, output: &mut Image<DP, U>) {
        _convert(self, output)
    }
}
