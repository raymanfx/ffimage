mod matrix {
    use ffimage::packed::generic::Matrix;

    #[test]
    fn new() {
        let matrix = Matrix::new(3, 3, 0u8);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.as_ref().len(), 3 * 3);
    }

    #[test]
    fn resize() {
        let mut matrix = Matrix::new(0, 0, 0u8);
        matrix.resize(2, 4, 0);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 4);
        assert_eq!(matrix.as_ref().len(), 2 * 4);
    }

    #[test]
    fn from_buf() {
        let mem = [0u8; 5];
        let matrix = Matrix::from_buf(&mem, 2, 2).expect("This is a valid matrix");
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(matrix.as_ref().len(), 2 * 2 + 1);
    }

    #[test]
    fn row_stride() {
        let mem = [0u16; 6];
        let matrix = Matrix::from_buf_with_stride(&mem, 2, 3, 2).expect("This is a valid matrix");
        assert_eq!(matrix.row_stride(), 3);
        assert_eq!(matrix.as_ref().len(), 6);
    }

    #[test]
    fn index() {
        let matrix = Matrix::new(2, 2, 2u8);
        assert_eq!(matrix[1], [2, 2]);
    }

    #[test]
    fn index_mut() {
        let mut matrix = Matrix::new(2, 2, 2u8);
        matrix[1][0] = matrix[1][0] + 1;
        assert_eq!(matrix[1], [3, 2]);
    }
}

mod image {
    use ffimage::color::*;
    use ffimage::core::{GenericImage, GenericImageView};
    use ffimage::packed::generic::Image;

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
        let image = Image::<Rgb<u8>, _>::from_buf_with_stride(&mem, 3, 3, 10)
            .expect("This is a valid image");
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
        let image = Image::<Rgb<u8>, _>::from_buf_with_stride(&mem, 3, 3, 10)
            .expect("This is a valid image");
        let pix = image.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }

    #[test]
    fn set_pixel() {
        let mut mem = vec![0; 27];
        let mut image = Image::<Rgb<u8>, _>::from_buf(&mut mem, 3, 3).unwrap();
        image
            .set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]))
            .unwrap();
        let pix = image.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);

        image
            .set_pixel(0, 2, &Rgb::<u8>::new([10, 20, 30]))
            .unwrap();
        let pix = image.pixel(0, 2).unwrap();
        assert_eq!(pix[0], 10);
        assert_eq!(pix[1], 20);
        assert_eq!(pix[2], 30);
    }
}
