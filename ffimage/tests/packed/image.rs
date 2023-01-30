use ffimage::color::*;
use ffimage::packed::Image;
use ffimage::traits::{GenericImage, GenericImageView};

#[test]
fn new() {
    let image = Image::<Rgb<u8>, _>::new(3, 3, 0u8);
    assert_eq!(image.width(), 3);
    assert_eq!(image.height(), 3);
    assert_eq!(image.as_ref().len(), 3 * 3 * 3);
}

#[test]
fn from_buf() {
    let mem = [3u16; 24];
    let image = Image::<Rgb<u16>, _>::from_buf(&mem, 2, 2).expect("This is a valid image");
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 2);
    assert_eq!(image.stride(), 2 * 3);
    assert_eq!(image.as_ref().len(), 2 * 2 * 3 * std::mem::size_of::<u16>());
}

#[test]
fn index() {
    let mut mem = vec![0; 27];
    mem[18] = 10;
    mem[19] = 20;
    mem[20] = 30;
    let image = Image::<Rgb<u8>, _>::from_buf(&mem, 3, 3).expect("This is a valid image");
    let pix = image[2][0];
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);

    let mut mem = vec![0; 30];
    mem[20] = 10;
    mem[21] = 20;
    mem[22] = 30;
    let image =
        Image::<Rgb<u8>, _>::from_buf_with_stride(&mem, 3, 3, 10).expect("This is a valid image");
    let pix = image[2][0];
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);
}

#[test]
fn pixel() {
    let mut mem = vec![0; 27];
    mem[18] = 10;
    mem[19] = 20;
    mem[20] = 30;
    let image = Image::<Rgb<u8>, _>::from_buf(&mem, 3, 3).expect("This is a valid image");
    let pix = image.pixel(0, 2).unwrap();
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);

    let mut mem = vec![0; 30];
    mem[20] = 10;
    mem[21] = 20;
    mem[22] = 30;
    let image =
        Image::<Rgb<u8>, _>::from_buf_with_stride(&mem, 3, 3, 10).expect("This is a valid image");
    let pix = image.pixel(0, 2).unwrap();
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);
}

#[test]
fn set_pixel() {
    let mut mem = vec![0; 27];
    let mut image = Image::<Rgb<u8>, _>::from_buf(&mut mem, 3, 3).unwrap();
    image.set_pixel(0, 2, &Rgb::<u8>([10, 20, 30])).unwrap();
    let pix = image.pixel(0, 2).unwrap();
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);

    image.set_pixel(0, 2, &Rgb::<u8>([10, 20, 30])).unwrap();
    let pix = image.pixel(0, 2).unwrap();
    assert_eq!(pix[0], 10);
    assert_eq!(pix[1], 20);
    assert_eq!(pix[2], 30);
}
