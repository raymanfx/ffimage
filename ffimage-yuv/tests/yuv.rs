use std::ops::RangeInclusive;

use ffimage::color::Rgb;
use ffimage::convert::Convert;
use ffimage::packed::Image;
use ffimage::traits::GenericImageView;

use ffimage_yuv::yuv::*;

fn make_range(val: u8, delta: u8) -> RangeInclusive<u8> {
    let lower = if val <= delta { 0 } else { val - delta };
    let upper = if val >= 255 - delta { 255 } else { val + delta };

    lower..=upper
}

#[test]
fn convert_yuv_rgb() {
    let mut mem: [u8; 12] = [10; 12];
    mem[0] = 11;
    mem[1] = 22;
    mem[2] = 33;
    mem[3] = 111;
    mem[4] = 222;
    mem[5] = 255;
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut yuv_buf = Image::<Yuv<u8>, _>::new(2, 2, 0u8);
    let mut rgb_buf = Image::<Rgb<u8>, _>::new(2, 2, 0u8);
    view.convert(&mut yuv_buf);
    yuv_buf.convert(&mut rgb_buf);

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.pixel(j, i).unwrap();
            let pix_out = rgb_buf.pixel(j, i).unwrap();
            let r_range = make_range(pix_in[0], 1);
            let g_range = make_range(pix_in[1], 1);
            let b_range = make_range(pix_in[2], 1);
            assert!(r_range.contains(&pix_out[0]));
            assert!(g_range.contains(&pix_out[1]));
            assert!(b_range.contains(&pix_out[2]));
        }
    }
}
