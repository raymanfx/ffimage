macro_rules! test_GenericImageView {
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
        fn index() {
            let mut mem = vec![0; 27];
            mem[18] = 10;
            mem[19] = 20;
            mem[20] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view[2][0];
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            mem[20] = 10;
            mem[21] = 20;
            mem[22] = 30;
            let view = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            let pix = view[2][0];
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

macro_rules! test_GenericImage {
    ($id:ident) => {
        #[test]
        fn index_mut() {
            let mut mem = vec![0; 27];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf[2][0];
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf[2][0];
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
        }

        #[test]
        fn set_pixel() {
            let mut mem = vec![0; 27];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);

            let mut mem = vec![0; 30];
            let mut buf = $id::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
            buf.set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30])).unwrap();
            let pix = buf.pixel(0, 2).unwrap();
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
        }
    };
}

mod view {
    use ffimage::color::*;
    use ffimage::core::GenericImageView;
    use ffimage::packed::generic::ImageView;

    test_GenericImageView!(ImageView);
}

mod flatbuffer {
    use ffimage::color::*;
    use ffimage::core::{GenericImage, GenericImageView};
    use ffimage::packed::generic::ImageViewMut;

    test_GenericImageView!(ImageViewMut);
    test_GenericImage!(ImageViewMut);
}

mod buffer {
    use ffimage::color::*;
    use ffimage::core::GenericImageView;
    use ffimage::packed::generic::{ImageBuffer, ImageView};

    #[test]
    fn from() {
        let mut mem = vec![0; 27];
        mem[18] = 10;
        mem[19] = 20;
        mem[20] = 30;
        let view = ImageView::<Rgb<u8>>::new(&mut mem, 3, 3).unwrap();
        let buffer = ImageBuffer::<Rgb<u8>>::from(&view);
        assert_eq!(buffer.width(), view.width());
        assert_eq!(buffer.height(), view.height());
        assert_eq!(buffer.stride(), (buffer.width() * 3) as usize);
        let pix = buffer.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    //test_ImageView!(GenericImageBuffer);
    //test_ImageBuffer!(GenericImageBuffer);
}
