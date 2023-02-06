use crate::yuv::Yuv;

/// YUV 4:2:0 (Planar)
///
/// This is a zero-sized struct, providing useful functions for handling planar YUV images.
/// The planar format is often used for video encoding / decoding usecases. Most cameras which
/// output YUV image frames will usually use a packet format, e.g. YUYV aka Y422.
pub struct Yuv420p;

impl Yuv420p {
    pub fn pack<'a, T>(buf: &'a [T]) -> impl IntoIterator<Item = Yuv<T>> + 'a
    where
        T: Copy,
    {
        // buf must have a rectangular shape
        assert_eq!(buf.len() % 2, 0);
        // buf must be divisible in 3 parts: 2/3 Luma (y) samples, 1/3 Chroma (u + v) samples
        assert_eq!(buf.len() % 3, 0);

        let pixels = (buf.len() / 3) * 2;
        let width = pixels / 4;

        let y = &buf[0..pixels];
        let u = &buf[pixels..(pixels + width)];
        let v = &buf[(pixels + width)..(pixels + width * 2)];

        Yuv420p::pack_planes(y, u, v)
    }

    pub fn pack_planes<'a, T>(
        y: &'a [T],
        u: &'a [T],
        v: &'a [T],
    ) -> impl IntoIterator<Item = Yuv<T>> + 'a
    where
        T: Copy,
    {
        // YUV420 has 2x2 blocks of Luma (y) samples
        assert_eq!(y.len() % 4, 0);
        let width = y.len() / 4;
        let height = y.len() / width;
        assert_eq!(u.len(), width);
        assert_eq!(v.len(), width);

        (0..height)
            .into_iter()
            .zip((0..width).into_iter())
            .map(move |(i, j)| {
                let y_idx = i * width + j;
                let uv_idx = i / 2 * width / 2 + j / 2;
                let y = y[y_idx];
                let u = u[uv_idx];
                let v = v[uv_idx];
                Yuv([y, u, v])
            })
    }
}
