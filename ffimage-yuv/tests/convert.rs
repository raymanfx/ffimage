use std::ops::RangeInclusive;

use ffimage::color::Rgb;

use ffimage_yuv::{
    yuv::Yuv,
    yuv422::{Yuv422, Yuyv},
};

fn make_range(val: u8, delta: u8) -> RangeInclusive<u8> {
    let lower = if val <= delta { 0 } else { val - delta };
    let upper = if val >= 255 - delta { 255 } else { val + delta };

    lower..=upper
}

#[test]
fn convert_convert_yuy_to_yuyv() {
    let yuv = vec![Yuv::<u8>([10, 10, 10]); 10];
    let yuyv: Vec<Yuyv<u8>> = yuv
        .iter()
        .copied()
        .zip(yuv.iter().copied().skip(1))
        .map(|(yuv1, yuv2)| Yuyv::<u8>::from([yuv1, yuv2]))
        .collect();

    (yuv.iter().copied().zip(yuv.iter().copied().skip(1)))
        .zip(yuyv.into_iter())
        .for_each(|((yuv1, yuv2), yuyv)| {
            // one macropixel is two image pixels
            assert_eq!(yuyv[0], yuv1[0]);
            assert_eq!(yuyv[1], yuv1[1]);
            assert_eq!(yuyv[2], yuv2[1]);
            assert_eq!(yuyv[3], yuv1[2]);
        });
}

#[test]
fn convert_convert_yuyv_to_yuv() {
    let yuyv = vec![Yuv422::<u8, 0, 2, 1, 3>([10, 10, 10, 10]); 10];
    let yuv: Vec<Yuv<u8>> = yuyv
        .iter()
        .copied()
        .map(|yuyv| <[Yuv<u8>; 2]>::from(yuyv))
        .flatten()
        .collect();

    yuyv.iter()
        .copied()
        .zip(yuv.iter().copied().zip(yuv.iter().copied().skip(1)))
        .for_each(|(yuyv, (yuv1, yuv2))| {
            // one macropixel is two image pixels
            assert_eq!(yuv1[0], yuyv[0]);
            assert_eq!(yuv1[1], yuyv[1]);
            assert_eq!(yuv2[0], yuyv[2]);
            assert_eq!(yuv1[2], yuyv[3]);
        });
}

#[test]
fn convert_rgb_to_yuv_to_rgb() {
    let rgb_in = vec![Rgb::<u8>([10, 10, 10]); 10];
    let yuv: Vec<Yuv<u8>> = rgb_in
        .iter()
        .copied()
        .map(|rgb| Yuv::<u8>::from(rgb))
        .collect();
    let rgb_out: Vec<Rgb<u8>> = yuv
        .iter()
        .copied()
        .map(|yuv| Rgb::<u8>::from(yuv))
        .collect();

    rgb_in
        .into_iter()
        .zip(rgb_out.into_iter())
        .for_each(|(rgb_in, rgb_out)| {
            let r_range = make_range(rgb_in[0], 1);
            let g_range = make_range(rgb_in[1], 1);
            let b_range = make_range(rgb_in[2], 1);
            assert!(r_range.contains(&rgb_out[0]));
            assert!(g_range.contains(&rgb_out[1]));
            assert!(b_range.contains(&rgb_out[2]));
        });
}
