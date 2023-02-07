use crate::yuv::Yuv;

/// YUV 4:2:0 (Planar)
///
/// This is a zero-sized struct, providing useful functions for handling planar YUV images.
/// The planar format is often used for video encoding / decoding usecases. Most cameras which
/// output YUV image frames will usually use a packet format, e.g. YUYV aka Y422.
pub struct Yuv420p;

impl Yuv420p {
    /// Returns packed Yuv444 color samples from a given slice.
    pub fn pack<'a, T>(buf: &'a [T]) -> impl IntoIterator<Item = Yuv<T>> + 'a
    where
        T: Copy,
    {
        Self::pack_bytes(buf)
            .into_iter()
            .map(|chunk| Yuv::<T>::from(chunk))
    }

    /// Returns packed Yuv444 color samples as byte chunks from a given slice.
    pub fn pack_bytes<'a, T>(buf: &'a [T]) -> impl IntoIterator<Item = [T; 3]> + 'a
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

    /// Returns packed Yuv444 color samples as byte chunks from YUV planes.
    pub fn pack_planes<'a, T>(
        y: &'a [T],
        u: &'a [T],
        v: &'a [T],
    ) -> impl IntoIterator<Item = [T; 3]> + 'a
    where
        T: Copy,
    {
        // YUV420 has 2x2 blocks of Luma (y) samples
        assert_eq!(y.len() % 4, 0);
        let width = y.len() / 4;
        let height = y.len() / width;
        assert_eq!(y.len(), width * height);
        assert_eq!(u.len(), width);
        assert_eq!(v.len(), width);

        (0..height)
            .into_iter()
            .zip((0..width).into_iter())
            .map(move |(i, j)| {
                let y_idx = i * width + j;
                let uv_idx = i / 2 * width / 2 + j / 2;
                [y[y_idx], u[uv_idx], v[uv_idx]]
            })
    }

    /// Unpacks packed Yuv444 color samples into Y, U, V planes.
    ///
    /// # Arguments
    ///
    /// * `yuv444` - Iterator generating Yuv444 samples
    /// * `y` - Luma plane
    /// * `u` - Chroma (blue) plane
    /// * `v` - Chroma (green) plane
    pub fn unpack<T>(
        yuv444: impl IntoIterator<Item = Yuv<T>>,
        y: &mut [T],
        u: &mut [T],
        v: &mut [T],
    ) where
        T: Copy,
    {
        Self::unpack_bytes(yuv444.into_iter().map(|pix| pix.0), y, u, v)
    }

    /// Unpacks packed Yuv444 color samples into Y, U, V planes.
    ///
    /// # Arguments
    ///
    /// * `yuv444` - Iterator generating Yuv444 byte chunks
    /// * `y` - Luma plane
    /// * `u` - Chroma (blue) plane
    /// * `v` - Chroma (green) plane
    pub fn unpack_bytes<T>(
        yuv444: impl IntoIterator<Item = [T; 3]>,
        y: &mut [T],
        u: &mut [T],
        v: &mut [T],
    ) where
        T: Copy,
    {
        // YUV420 has 2x2 blocks of Luma (y) samples
        assert_eq!(y.len() % 4, 0);
        let width = y.len() / 4;
        let height = y.len() / width;
        assert_eq!(y.len(), width * height);
        assert_eq!(u.len(), width);
        assert_eq!(v.len(), width);

        yuv444.into_iter().enumerate().for_each(|(i, yuv)| {
            y[i] = yuv[0];
            u[i] = yuv[1];
            v[i] = yuv[2];
        })
    }
}
