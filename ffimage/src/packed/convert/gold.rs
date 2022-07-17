use crate::convert::{Convert, MapPixels};
use crate::packed::Image;
use crate::traits::{GenericImageView, Pixel};

fn _convert<'a, SP, DP, T, U>(input: &Image<SP, T>, output: &mut Image<DP, U>)
where
    SP: 'a + Pixel + Copy + MapPixels<SP, DP>,
    DP: 'a + Pixel + Copy,
    T: AsRef<[SP::T]>,
    U: AsRef<[DP::T]> + AsMut<[DP::T]>,
{
    let rows = if input.height() < output.height() {
        input.height() as usize
    } else {
        output.height() as usize
    };

    (0..rows)
        .into_iter()
        .for_each(|i| SP::map_pixels(input[i].as_ref(), output[i].as_mut()))
}

impl<'a, SP, DP, T, U> Convert<Image<DP, U>> for Image<SP, T>
where
    SP: 'a + Pixel + Copy + MapPixels<SP, DP>,
    DP: 'a + Pixel + Copy,
    T: AsRef<[SP::T]>,
    U: AsRef<[DP::T]> + AsMut<[DP::T]>,
{
    fn convert(&self, output: &mut Image<DP, U>) {
        _convert(self, output)
    }
}
