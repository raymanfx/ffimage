macro_rules! test_GenericImageView {
    ($id:ident) => {
        #[test]
        fn as_slice() {
            let mut mem = vec![1, 2, 3];
            let view = $id::<Rgb<u8>>::new(&mut mem, 1, 1).unwrap();
            assert_eq!(view.as_slice()[0], 1);
            assert_eq!(view.as_slice()[1], 2);
            assert_eq!(view.as_slice()[2], 3);
        }

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

        #[test]
        fn view() {
            // 4x4 RGB with one padding byte at the end of each row
            let mut mem = vec![0; 52];
            mem[16] = 10;
            mem[17] = 20;
            mem[18] = 30;
            mem[32] = 11;
            mem[33] = 21;
            mem[34] = 31;
            let view = $id::<Rgb<u8>>::new(&mut mem, 4, 4).unwrap();
            let sub = view.view(1, 1, 2, 2).unwrap();
            let pix = sub[0][0];
            assert_eq!(pix[0], 10);
            assert_eq!(pix[1], 20);
            assert_eq!(pix[2], 30);
            let pix = sub[1][1];
            assert_eq!(pix[0], 11);
            assert_eq!(pix[1], 21);
            assert_eq!(pix[2], 31);
        }
    };
}

macro_rules! test_GenericImage {
    ($id:ident) => {
        #[test]
        fn as_mut_slice() {
            let mut mem = vec![1, 2, 3];
            let mut view = $id::<Rgb<u8>>::new(&mut mem, 1, 1).unwrap();
            assert_eq!(view.as_mut_slice()[0], 1);
            assert_eq!(view.as_mut_slice()[1], 2);
            assert_eq!(view.as_mut_slice()[2], 3);
        }

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
    fn into_vec() {
        let mut buf = ImageBuffer::<Rgb<u8>>::new(1, 1);
        buf[0][0][0] = 1;
        buf[0][0][1] = 2;
        buf[0][0][2] = 3;
        let vec = buf.into_vec();
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

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
