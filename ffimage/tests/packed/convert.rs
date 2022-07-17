use ffimage::color::*;
use ffimage::convert::Convert;
use ffimage::packed::Image;
use ffimage::traits::GenericImageView;

#[test]
fn convert_rgb_to_gray() {
    let mem: [u8; 12] = [10; 12];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Gray<u8>, _>::new(2, 2, 0u8);
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
    let view = Image::<Gray<u16>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Rgb<u16>, _>::new(2, 2, 0u16);
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
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Bgra<u8>, _>::new(2, 2, 0u8);
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
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 2, 2).unwrap();
    let mut buf = Image::<Gray<u8>, _>::new(1, 1, 0u8);
    view.convert(&mut buf);
}
