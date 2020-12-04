use std::convert::TryFrom;

use ffimage::color::*;
use ffimage::core::{Convert, GenericImageView};
use ffimage::packed::dynamic::ImageView as DynamicView;
use ffimage::packed::generic::{ImageBuffer, ImageView, ImageViewMut};

#[test]
fn convert_rgb_to_gray() {
    let mem: [u8; 12] = [10; 12];
    let view = ImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut buf = ImageBuffer::<Gray<u8>>::new(0, 0);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.pixel(j, i).unwrap();
            let pix_out = buf.pixel(j, i).unwrap();

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
    let view = ImageView::<Gray<u16>>::new(&mem, 2, 2).unwrap();
    let mut buf = ImageBuffer::<Rgb<u16>>::new(0, 0);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.pixel(j, i).unwrap();
            let pix_out = buf.pixel(j, i).unwrap();
            assert_eq!(pix_out, Rgb::<u16>::new([pix_in[0], pix_in[0], pix_in[0]]));
        }
    }
}

#[test]
fn convert_rgb_to_bgra() {
    let mem: [u8; 12] = [10, 20, 30, 11, 21, 31, 12, 22, 32, 13, 23, 33];
    let view = ImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut mem_flat: [u8; 16] = [0; 16];
    let mut buf = ImageViewMut::<Bgra<u8>>::new(&mut mem_flat, 2, 2).unwrap();
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in 0..view.width() {
            let pix_in = view.pixel(j, i).unwrap();
            let pix_out = buf.pixel(j, i).unwrap();
            assert_eq!(
                pix_out,
                Bgra::<u8>::new([pix_in[2], pix_in[1], pix_in[0], 255])
            );
        }
    }
}

#[test]
fn convert_rgb_to_gray_partial() {
    let mem: [u8; 12] = [10, 20, 30, 11, 21, 31, 12, 22, 32, 13, 23, 33];
    let view = ImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut mem_flat: [u8; 1] = [0; 1];
    let mut buf = ImageViewMut::<Gray<u8>>::new(&mut mem_flat, 1, 1).unwrap();
    view.convert(&mut buf);
}

#[test]
fn convert_dynamic_to_gray() {
    let mem: [u8; 12] = [10; 12];
    let dynamic_view = DynamicView::new(&mem, 2, 2).unwrap();
    let generic_view = ImageView::<Rgb<u8>>::try_from(&dynamic_view).unwrap();
    let mut buf = ImageBuffer::<Gray<u8>>::new(0, 0);
    generic_view.convert(&mut buf);

    for i in 0..generic_view.height() {
        for j in 0..generic_view.width() {
            let pix_in = generic_view.pixel(j, i).unwrap();
            let pix_out = buf.pixel(j, i).unwrap();

            // rec601 luma
            let y = (0.2126 * pix_in[0] as f32
                + 0.7152 * pix_in[1] as f32
                + 0.0722 * pix_in[2] as f32) as u8;
            assert_eq!(pix_out, Gray::<u8>::new([y]));
        }
    }
}
