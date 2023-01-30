extern crate ffimage;

use ffimage::color::gray::*;
use ffimage::color::rgb::*;

#[test]
fn from_gray() {
    let src = Gray([111u8]);
    let dst: Rgb<u8> = Rgb::from(src);

    assert_eq!(dst[0], src[0]);
    assert_eq!(dst[1], src[0]);
    assert_eq!(dst[2], src[0]);
}

#[test]
fn from_bgr() {
    let src = Rgb::<u8, 2, 1, 0>([111u8; 3]);
    let dst = Rgb::<u8>::from(src);

    assert_eq!(dst[0], src[2]);
    assert_eq!(dst[1], src[1]);
    assert_eq!(dst[2], src[0]);
}
