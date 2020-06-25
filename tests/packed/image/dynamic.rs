mod view {
    use ffimage::packed::DynamicImageView;

    #[test]
    fn new() {
        let mem = vec![0; 27];
        let view = DynamicImageView::new(&mem, 3, 3, 3 /* channels */).unwrap();
        assert_eq!(view.raw.len(), 3 * 3 * 3);
        assert_eq!(view.width, 3);
        assert_eq!(view.height, 3);
        assert_eq!(view.stride, 3 * 3);

        let mem = vec![0; 30];
        let view = DynamicImageView::new(&mem, 3, 3, 3 /* channels */).unwrap();
        assert_eq!(view.raw.len(), 3 * 3 * 3 + 3);
        assert_eq!(view.width, 3);
        assert_eq!(view.height, 3);
        assert_eq!(view.stride, 3 * 3 + 1);
    }

    #[test]
    fn with_stride() {
        let mem = vec![0; 30];
        let view = DynamicImageView::with_stride(&mem, 3, 3, 3 * 3 + 1 /* stride */).unwrap();
        assert_eq!(view.raw.len(), 3 * 3 * 3 + 3);
        assert_eq!(view.width, 3);
        assert_eq!(view.height, 3);
        assert_eq!(view.stride, 3 * 3 + 1);
    }
}

mod buffer {
    use ffimage::packed::{DynamicImageBuffer, DynamicStorageType};

    #[test]
    fn new() {
        let buf = DynamicImageBuffer::new(3, 3, 3 /* channels */, DynamicStorageType::U8);
        assert_eq!(buf.raw.len(), 3 * 3 * 3);
        assert_eq!(buf.width, 3);
        assert_eq!(buf.height, 3);
        assert_eq!(buf.stride, 3 * 3);
    }

    #[test]
    fn with_raw() {
        let mem = vec![0; 27];
        let buf = DynamicImageBuffer::with_raw(3, 3, 3, &mem);
        assert_eq!(buf.raw.len(), 3 * 3 * 3);
        assert_eq!(buf.width, 3);
        assert_eq!(buf.height, 3);
        assert_eq!(buf.stride, 3 * 3);
    }

    #[test]
    fn resize() {
        let mut buf = DynamicImageBuffer::new(0, 0, 3 /* channels */, DynamicStorageType::U8);
        buf.resize(3, 3, 3);
        assert_eq!(buf.width, 3);
        assert_eq!(buf.height, 3);
        assert_eq!(buf.stride, 3 * 3);
    }
}
