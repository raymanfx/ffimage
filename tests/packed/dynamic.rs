mod view {
    use ffimage::packed::dynamic::ImageView;

    #[test]
    fn new() {
        let mem: Vec<u8> = vec![0; 27];
        let view = ImageView::new(&mem, 3, 3).unwrap();
        assert_eq!(view.raw().len(), 3 * 3 * 3);
        assert_eq!(view.width(), 3);
        assert_eq!(view.height(), 3);
        assert_eq!(view.stride(), 3 * 3);

        let mem: Vec<u16> = vec![0; 30];
        let view = ImageView::new(&mem, 3, 3).unwrap();
        assert_eq!(view.raw().len(), 3 * 3 * 3 + 3);
        assert_eq!(view.width(), 3);
        assert_eq!(view.height(), 3);
        assert_eq!(view.stride(), 3 * 3 + 1);
    }

    #[test]
    fn with_stride() {
        let mem: Vec<u8> = vec![0; 30];
        let view = ImageView::with_stride(&mem, 3, 3, 3 * 3 + 1 /* stride */).unwrap();
        assert_eq!(view.raw().len(), 3 * 3 * 3 + 3);
        assert_eq!(view.width(), 3);
        assert_eq!(view.height(), 3);
        assert_eq!(view.stride(), 3 * 3 + 1);
    }
}

mod buffer {
    use ffimage::packed::dynamic::{ImageBuffer, StorageType};

    #[test]
    fn new() {
        let buf = ImageBuffer::new(3, 3, 3 /* channels */, StorageType::U8);
        assert_eq!(buf.raw().len(), 3 * 3 * 3);
        assert_eq!(buf.width(), 3);
        assert_eq!(buf.height(), 3);
        assert_eq!(buf.stride(), 3 * 3);
    }

    #[test]
    fn from_raw() {
        let mem: Vec<u8> = vec![0; 27];
        let buf = ImageBuffer::from_raw(3, 3, mem).unwrap();
        assert_eq!(buf.raw().len(), 3 * 3 * 3);
        assert_eq!(buf.width(), 3);
        assert_eq!(buf.height(), 3);
        assert_eq!(buf.stride(), 3 * 3);
    }

    #[test]
    fn resize() {
        let mut buf = ImageBuffer::empty(StorageType::U8);
        buf.resize(3, 3, 3);
        assert_eq!(buf.width(), 3);
        assert_eq!(buf.height(), 3);
        assert_eq!(buf.stride(), 3 * 3);
        assert_eq!(buf.raw().len(), 3 * 3 * 3);
    }
}
