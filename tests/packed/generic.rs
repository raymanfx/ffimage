macro_rules! test_ImageView {
    ($id:ident) => {
        #[test]
        fn width() {
            let mut mem = vec![0; 27];
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            assert_eq!(view.width(), 3);
        }

        #[test]
        fn height() {
            let mut mem = vec![0; 27];
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            assert_eq!(view.height(), 3);
        }

        #[test]
        fn stride() {
            let mut mem = vec![0; 27];
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            assert_eq!(view.stride(), 3 * 3);

            let mut mem = vec![0; 30];
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            assert_eq!(view.stride(), 3 * 3 + 1);
        }

        #[test]
        fn get_pixel() {
            let mut mem = vec![0; 27];
            mem[18] = 10;
            mem[19] = 20;
            mem[20] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view.get_pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            mem[20] = 10;
            mem[21] = 20;
            mem[22] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view.get_pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
        }

        #[test]
        fn pixel_row() {
            let mut mem = vec![0; 12];
            mem[6] = 10;
            mem[7] = 20;
            mem[8] = 30;
            mem[9] = 11;
            mem[10] = 21;
            mem[11] = 31;
            let view = $id::<Rgb<u8>>::new(&mut mem, 2, 2).unwrap();
            let row = view.pixel_row(1).unwrap();
            assert_eq!(row[0], Rgb::<u8>::new([10, 20, 30]));
            assert_eq!(row[1], Rgb::<u8>::new([11, 21, 31]));

            let mut mem = vec![0; 14];
            mem[7] = 10;
            mem[8] = 20;
            mem[9] = 30;
            mem[10] = 11;
            mem[11] = 21;
            mem[12] = 31;
            let view = $id::<Rgb<u8>>::new(&mut mem, 2, 2).unwrap();
            let row = view.pixel_row(1).unwrap();
            assert_eq!(row[0], Rgb::<u8>::new([10, 20, 30]));
            assert_eq!(row[1], Rgb::<u8>::new([11, 21, 31]));
        }

        #[test]
        fn pixel() {
            let mut mem = vec![0; 27];
            mem[18] = 10;
            mem[19] = 20;
            mem[20] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            mem[20] = 10;
            mem[21] = 20;
            mem[22] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
        }
    };
}

macro_rules! test_ImageBuffer {
    ($id:ident) => {
        #[test]
        fn set_pixel() {
            let mut mem = vec![0; 27];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf.get_pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf.get_pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
        }

        #[test]
        fn pixel_row_mut() {
            let mut mem = vec![0; 12];
            mem[6] = 10;
            mem[7] = 20;
            mem[8] = 30;
            mem[9] = 11;
            mem[10] = 21;
            mem[11] = 31;
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 2, 2).unwrap();
            let row = buf.pixel_row_mut(1).unwrap();
            assert_eq!(row[0], Rgb::<u8>::new([10, 20, 30]));
            assert_eq!(row[1], Rgb::<u8>::new([11, 21, 31]));

            let mut mem = vec![0; 14];
            mem[7] = 10;
            mem[8] = 20;
            mem[9] = 30;
            mem[10] = 11;
            mem[11] = 21;
            mem[12] = 31;
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 2, 2).unwrap();
            let row = buf.pixel_row_mut(1).unwrap();
            assert_eq!(row[0], Rgb::<u8>::new([10, 20, 30]));
            assert_eq!(row[1], Rgb::<u8>::new([11, 21, 31]));
        }

        #[test]
        fn pixel_mut() {
            let mut mem = vec![0; 27];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = buf.pixel_mut(0, 2).unwrap();
            pix[0] = 11;
            pix[1] = 22;
            pix[2] = 33;
            let pix = buf.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 11);
            assert_eq!(pix[1], 22);
            assert_eq!(pix[2], 33);

            let mut mem = vec![0; 30];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = buf.pixel_mut(0, 2).unwrap();
            pix[0] = 11;
            pix[1] = 22;
            pix[2] = 33;
            let pix = buf.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 11);
            assert_eq!(pix[1], 22);
            assert_eq!(pix[2], 33);
        }
    };
}

mod view {
    use ffimage::color::*;
    use ffimage::core::ImageView;
    use ffimage::packed::{AccessPixel, GenericImageView};

    test_ImageView!(GenericImageView);
}

mod flatbuffer {
    use ffimage::color::*;
    use ffimage::core::{ImageBuffer, ImageView};
    use ffimage::packed::{AccessPixel, AccessPixelMut, GenericImageFlatBuffer};

    test_ImageView!(GenericImageFlatBuffer);
    test_ImageBuffer!(GenericImageFlatBuffer);
}

mod buffer {
    use ffimage::color::*;
    use ffimage::core::ImageView;
    use ffimage::packed::{GenericImageBuffer, GenericImageView};

    #[test]
    fn from() {
        let mut mem = vec![0; 27];
        mem[18] = 10;
        mem[19] = 20;
        mem[20] = 30;
        let view = GenericImageView::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        let buffer = GenericImageBuffer::<Rgb<u8>>::from(&view);
        assert_eq!(buffer.width(), view.width());
        assert_eq!(buffer.height(), view.height());
        assert_eq!(buffer.stride(), (buffer.width() * 3) as usize);
        let pix = buffer.get_pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    //test_ImageView!(GenericImageBuffer);
    //test_ImageBuffer!(GenericImageBuffer);
}
