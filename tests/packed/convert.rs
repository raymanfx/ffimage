use std::convert::TryFrom;

use ffimage::color::*;
use ffimage::core::{ImageView, TryConvert};
use ffimage::packed::{
    DynamicImageView, GenericImageBuffer, GenericImageFlatBuffer, GenericImageView,
};

#[test]
fn convert_rgb_to_gray() {
    let mem: [u8; 12] = [10; 12];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut buf = GenericImageBuffer::<Gray<u8>>::new(0, 0);
    view.try_convert(&mut buf).unwrap();

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.get_pixel(j, i).unwrap();
            let pix_out = buf.get_pixel(j, i).unwrap();

            // rec601 luma
            let y = (0.2126 * pix_in[0] as f32
                + 0.7152 * pix_in[1] as f32
                + 0.0722 * pix_in[2] as f32) as u8;
            assert_eq!(pix_out, Gray::<u8>::new([y]));
        }
    }
}

#[test]
fn convert_gray_to_rgb() {
    let mem: [u16; 4] = [0; 4];
    let view = GenericImageView::<Gray<u16>>::new(&mem, 2, 2).unwrap();
    let mut buf = GenericImageBuffer::<Rgb<u16>>::new(0, 0);
    view.try_convert(&mut buf).unwrap();

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.get_pixel(j, i).unwrap();
            let pix_out = buf.get_pixel(j, i).unwrap();
            assert_eq!(pix_out, Rgb::<u16>::new([pix_in[0], pix_in[0], pix_in[0]]));
        }
    }
}

#[test]
fn try_convert_rgb_to_bgra() {
    let mem: [u8; 12] = [10, 20, 30, 11, 21, 31, 12, 22, 32, 13, 23, 33];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut mem_flat: [u8; 16] = [0; 16];
    let mut buf = GenericImageFlatBuffer::<Bgra<u8>>::new(&mut mem_flat, 2, 2).unwrap();
    view.try_convert(&mut buf).unwrap();

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.get_pixel(j, i).unwrap();
            let pix_out = buf.get_pixel(j, i).unwrap();
            assert_eq!(
                pix_out,
                Bgra::<u8>::new([pix_in[2], pix_in[1], pix_in[0], 0])
            );
        }
    }
}

#[test]
fn try_convert_rgb_to_gray() {
    let mem: [u8; 12] = [10, 20, 30, 11, 21, 31, 12, 22, 32, 13, 23, 33];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut mem_flat: [u8; 3] = [0; 3];
    let mut buf = GenericImageFlatBuffer::<Gray<u8>>::new(&mut mem_flat, 2, 1).unwrap();
    let result = view.try_convert(&mut buf);
    assert!(result.is_err());
}

#[test]
fn try_convert_dynamic_to_gray() {
    let mem: [u8; 12] = [10; 12];
    let dynamic_view = DynamicImageView::new(&mem, 2, 2, 3).unwrap();
    let generic_view = GenericImageView::<Rgb<u8>>::try_from(&dynamic_view).unwrap();
    let mut buf = GenericImageBuffer::<Gray<u8>>::new(0, 0);
    generic_view.try_convert(&mut buf).unwrap();

    for i in 0..generic_view.height() {
        for j in 0..generic_view.width() {
            let pix_in = generic_view.get_pixel(j, i).unwrap();
            let pix_out = buf.get_pixel(j, i).unwrap();

            // rec601 luma
            let y = (0.2126 * pix_in[0] as f32
                + 0.7152 * pix_in[1] as f32
                + 0.0722 * pix_in[2] as f32) as u8;
            assert_eq!(pix_out, Gray::<u8>::new([y]));
        }
    }
}
