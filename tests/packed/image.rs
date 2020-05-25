mod view {
    use ffimage::color::*;
    use ffimage::core::ImageView;
    use ffimage::packed::{AccessPixel, GenericImageView};

    #[test]
    fn width() {
        let mem = vec![0; 27];
        let view = GenericImageView::<Rgb<u8>>::new(&mem, 3, 3).unwrap();
        assert_eq!(view.width(), 3);
    }

    #[test]
    fn height() {
        let mem = vec![0; 27];
        let view = GenericImageView::<Rgb<u8>>::new(&mem, 3, 3).unwrap();
        assert_eq!(view.height(), 3);
    }

    #[test]
    fn stride() {
        let mem = vec![0; 27];
        let view = GenericImageView::<Rgb<u8>>::new(&mem, 3, 3).unwrap();
        assert_eq!(view.stride(), 3 * 3);

        let mem = vec![0; 30];
        let view = GenericImageView::<Rgb<u8>>::with_stride(&mem, 3, 3, 10).unwrap();
        assert_eq!(view.stride(), 3 * 3 + 1);
    }

    #[test]
    fn get_pixel() {
        let mut mem = vec![0; 27];
        mem[18] = 10;
        mem[19] = 20;
        mem[20] = 30;
        let view = GenericImageView::<Rgb<u8>>::new(&mem, 3, 3).unwrap();
        let pix = view.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        let mut mem = vec![0; 30];
        mem[20] = 10;
        mem[21] = 20;
        mem[22] = 30;
        let view = GenericImageView::<Rgb<u8>>::with_stride(&mem, 3, 3, 10).unwrap();
        let pix = view.get_pixel(0, 2).unwrap();
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
        let view = GenericImageView::<Rgb<u8>>::new(&mem, 3, 3).unwrap();
        let pix = view.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        let mut mem = vec![0; 30];
        mem[20] = 10;
        mem[21] = 20;
        mem[22] = 30;
        let view = GenericImageView::<Rgb<u8>>::with_stride(&mem, 3, 3, 10).unwrap();
        let pix = view.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }
}

mod flatbuffer {
    use ffimage::color::*;
    use ffimage::core::{ImageBuffer, ImageView};
    use ffimage::packed::{AccessPixel, AccessPixelMut, GenericImageFlatBuffer};

    #[test]
    fn width() {
        let mut mem = vec![0; 27];
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        assert_eq!(buf.width(), 3);
    }

    #[test]
    fn height() {
        let mut mem = vec![0; 27];
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        assert_eq!(buf.height(), 3);
    }

    #[test]
    fn stride() {
        let mut mem = vec![0; 27];
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        assert_eq!(buf.stride(), 3 * 3);

        let mut mem = vec![0; 30];
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 3, 3, 10).unwrap();
        assert_eq!(buf.stride(), 3 * 3 + 1);
    }

    #[test]
    fn get_pixel() {
        let mut mem = vec![0; 27];
        mem[18] = 10;
        mem[19] = 20;
        mem[20] = 30;
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        let pix = buf.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        let mut mem = vec![0; 30];
        mem[20] = 10;
        mem[21] = 20;
        mem[22] = 30;
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 3, 3, 10).unwrap();
        let pix = buf.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn set_pixel() {
        let mut mem = vec![0; 27];
        let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]));
        let pix = buf.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        let mut mem = vec![0; 30];
        let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 3, 3, 10).unwrap();
        buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]));
        let pix = buf.get_pixel(0, 2).unwrap();
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
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        let mut mem = vec![0; 30];
        mem[20] = 10;
        mem[21] = 20;
        mem[22] = 30;
        let buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 3, 3, 10).unwrap();
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn pixel_mut() {
        let mut mem = vec![0; 27];
        let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        let pix = buf.pixel_mut(0, 2).unwrap();
        pix[0] = 11;
        pix[1] = 22;
        pix[2] = 33;
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 11);
        assert_eq!(pix[1], 22);
        assert_eq!(pix[2], 33);

        let mut mem = vec![0; 30];
        let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 3, 3, 10).unwrap();
        let pix = buf.pixel_mut(0, 2).unwrap();
        pix[0] = 11;
        pix[1] = 22;
        pix[2] = 33;
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 11);
        assert_eq!(pix[1], 22);
        assert_eq!(pix[2], 33);
    }
}

mod buffer {
    use ffimage::color::*;
    use ffimage::core::{ImageBuffer, ImageView, Resize};
    use ffimage::packed::{AccessPixel, AccessPixelMut, GenericImageBuffer};

    #[test]
    fn width() {
        let buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        assert_eq!(buf.width(), 3);
    }

    #[test]
    fn height() {
        let buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        assert_eq!(buf.height(), 3);
    }

    #[test]
    fn stride() {
        let buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        assert_eq!(buf.stride(), 3 * 3);
    }

    #[test]
    fn get_pixel() {
        let mut mem = vec![0; 27];
        mem[18] = 10;
        mem[19] = 20;
        mem[20] = 30;
        let buf = GenericImageBuffer::<Rgb<u8>>::with_raw(3, 3, &mem).unwrap();
        let pix = buf.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn set_pixel() {
        let mut buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]));
        let pix = buf.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn resize() {
        let mut buf = GenericImageBuffer::<Rgb<u8>>::new(0, 0);
        assert_eq!(buf.width(), 0);
        assert_eq!(buf.height(), 0);
        buf.resize(3, 3);
        assert_eq!(buf.width(), 3);
        assert_eq!(buf.height(), 3);
        assert_eq!(buf.stride(), 3 * 3);
    }

    #[test]
    fn pixel() {
        let mut buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]));
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn pixel_mut() {
        let mut buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
        let pix = buf.pixel_mut(0, 2).unwrap();
        pix[0] = 11;
        pix[1] = 22;
        pix[2] = 33;
        let pix = buf.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 11);
        assert_eq!(pix[1], 22);
        assert_eq!(pix[2], 33);
    }
}
