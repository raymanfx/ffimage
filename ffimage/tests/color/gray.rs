extern crate ffimage;

use num_traits::cast::FromPrimitive;

use ffimage::color::gray::*;
use ffimage::color::rgb::*;

#[test]
fn from_rgb() {
    let src = Rgb::<u8>([111u8; 3]);
    let dst = Gray::<u8>::from(src);

    let y = u8::from_f32(0.2126 * src[0] as f32 + 0.7152 * src[1] as f32 + 0.0722 * src[2] as f32)
        .unwrap();
    assert_eq!(dst[0], y);
}
