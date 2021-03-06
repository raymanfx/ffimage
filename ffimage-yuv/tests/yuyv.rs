use ffimage::packed::Image;
use ffimage::traits::{Convert, GenericImageView};

use ffimage_yuv::yuv::*;
use ffimage_yuv::yuyv::*;

#[test]
fn convert_yuy_to_yuyv() {
    let mem: [u8; 12] = [10; 12];
    let view = Image::<Yuv<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Yuyv<u8>, _>::new(0, 0, 0u8);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in (0..view.width()).step_by(2) {
            let pix_in = [view.pixel(j, i).unwrap(), view.pixel(j + 1, i).unwrap()];
            let pix_out = buf.pixel(j, i).unwrap();

            // one macropixel is two image pixels
            assert_eq!(pix_out[0], pix_in[0][0]);
            assert_eq!(pix_out[1], pix_in[0][1]);
            assert_eq!(pix_out[2], pix_in[0][1]);
            assert_eq!(pix_out[3], pix_in[0][2]);
        }
    }
}

#[test]
fn convert_yuyv_to_yuv() {
    let mem: [u8; 8] = [10; 8];
    let view = Image::<Yuyv<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Yuv<u8>, _>::new(0, 0, 0u8);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in (0..view.width()).step_by(2) {
            let pix_in = view.pixel(j, i).unwrap();
            let pix_out = [buf.pixel(j, i).unwrap(), buf.pixel(j + 1, i).unwrap()];

            // one macropixel is two image pixels
            assert_eq!(pix_out[0][0], pix_in[0]);
            assert_eq!(pix_out[0][1], pix_in[1]);
            assert_eq!(pix_out[1][0], pix_in[2]);
            assert_eq!(pix_out[0][2], pix_in[3]);
        }
    }
}
