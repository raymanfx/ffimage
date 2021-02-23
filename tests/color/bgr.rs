extern crate ffimage;

use ffimage::color::bgr::*;
use ffimage::color::gray::*;
use ffimage::color::rgb::*;

#[test]
fn from_gray() {
    let src_mem = [111u8; 1];
    let src = Gray::<u8>::from(src_mem);
    let dst = Bgr::<u8>::from(src);

    assert_eq!(dst[0], src[0]);
    assert_eq!(dst[1], src[0]);
    assert_eq!(dst[2], src[0]);
}

#[test]
fn from_rgb() {
    let src_mem = [111u8; 3];
    let src = Rgb::<u8>::from(src_mem);
    let dst = Bgr::<u8>::from(src);

    assert_eq!(dst[0], src[2]);
    assert_eq!(dst[1], src[1]);
    assert_eq!(dst[2], src[0]);
}
