use core::ops::{Index, IndexMut};

use std::mem;

use num_traits::identities::Zero;

use crate::core::iter::{PixelIter, PixelIterMut};
use crate::core::traits::{GenericImage, GenericImageView, Pixel};

macro_rules! impl_GenericImageView {
    ($id:ident) => {
        type T = T;

        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn pixel(&self, x: u32, y: u32) -> Option<Self::T> {
            if x >= self.width() || y >= self.height() {
                return None;
            }

            Some(self[y as usize][x as usize])
        }
    };
}

macro_rules! impl_GenericImage {
    ($id:ident) => {
        fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), ()> {
            if x >= self.width() || y >= self.height() {
                return Err(());
            }

            self[y as usize][x as usize] = *pix;
            Ok(())
        }
    };
}

macro_rules! impl_Index {
    ($id:ident) => {
        type Output = [T];

        fn index(&self, index: usize) -> &Self::Output {
            // determine the offset in the raw buffer
            let width = self.width();
            let stride_elems = self.stride() / mem::size_of::<T::T>();
            let off: usize = index * stride_elems;
            let slice = &self.raw[off..off + stride_elems];
            let (head, body, _tail) = unsafe { slice.align_to::<T>() };
            assert!(head.is_empty(), "raw data is not aligned");
            assert_eq!(body.len(), width as usize, "invalid number of row items");

            body
        }
    };
}

macro_rules! impl_IndexMut {
    ($id:ident) => {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            // determine the offset in the raw buffer
            let width = self.width();
            let stride_elems = self.stride() / mem::size_of::<T::T>();
            let off: usize = index * stride_elems;
            let slice = &mut self.raw[off..off + stride_elems];
            let (head, body, _tail) = unsafe { slice.align_to_mut::<T>() };
            assert!(head.is_empty(), "raw data is not aligned");
            assert_eq!(body.len(), width as usize, "invalid number of row items");

            body
        }
    };
}

macro_rules! impl_Iterator {
    ($id:ident) => {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.x >= self.width {
                self.x = 0;
                self.y += 1;
            }

            if self.y >= self.height {
                return None;
            }

            let x = self.x;
            self.x += 1;

            Some(&self.img[self.y as usize][x as usize])
        }
    };
}

// iterators handing out mutable references are not allowed by safe rust as explained here:
// https://stackoverflow.com/a/27641876/11423991
macro_rules! impl_IteratorMut {
    ($id:ident) => {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.x >= self.width {
                self.x = 0;
                self.y += 1;
            }

            if self.y >= self.height {
                return None;
            }

            let x = self.x;
            self.x += 1;

            // This is safe because...
            // (from http://stackoverflow.com/questions/25730586):
            // The Rust compiler does not know that when you ask a mutable iterator for the
            // next element, that you get a different reference every time and never the same
            // reference twice. Of course, we know that such an iterator won't give you the
            // same reference twice.
            unsafe { mem::transmute(&self.img[self.y as usize][x as usize]) }
        }
    };
}

#[derive(Debug, Clone, Copy)]
/// Image view parametrized by its pixel type
pub struct ImageView<'a, T: Pixel> {
    raw: &'a [T::T],
    width: u32,
    height: u32,
    stride: usize,
}

impl<'a, T: Pixel> ImageView<'a, T> {
    /// Returns an image view with pixel accessors
    ///
    /// The backing memory storage must have the same element type as the underlying pixel type of
    /// the image.
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::packed::generic::ImageView;
    ///
    /// let mem = vec![0; 3];
    /// let view = ImageView::<Rgb<u8>>::new(&mem, 1, 1).expect("Memory region too small");
    /// ```
    pub fn new(raw: &'a [T::T], width: u32, height: u32) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(ImageView {
            raw,
            width,
            height,
            stride,
        })
    }

    #[deprecated(since = "0.8.2", note = "Please use the as_slice() function instead")]
    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    /// Returns a view to the pixel backing storage
    pub fn as_slice(&self) -> &[T::T] {
        &self.raw
    }

    /// Returns a mutable view to the pixel backing storage
    pub fn as_mut_slice(&mut self) -> &[T::T] {
        &mut self.raw
    }

    /// Returns the length of one pixel row in bytes
    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl<'a, T: Pixel + 'a> GenericImageView<'a> for ImageView<'a, T> {
    impl_GenericImageView!(ImageView);

    type SubImage = SubImageView<'a, Self>;

    fn view(&'a self, x: u32, y: u32, width: u32, height: u32) -> Option<Self::SubImage> {
        SubImageView::new(self, x, y, width, height)
    }
}

impl<'a, T: Pixel> Index<usize> for ImageView<'a, T> {
    impl_Index!(ImageView);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, ImageView<'a, T>> {
    impl_Iterator!(ImageView);
}

impl<'a, T: Pixel> IntoIterator for &'a ImageView<'a, T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, ImageView<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

#[derive(Debug)]
/// Mutable image view
pub struct ImageViewMut<'a, T: Pixel> {
    raw: &'a mut [T::T],
    width: u32,
    height: u32,
    stride: usize,
}

impl<'a, T: Pixel> ImageViewMut<'a, T> {
    /// Returns a flat image buffer with pixel accessors
    ///
    /// 'Flat' means that the backing memory of the image is not allocated by the struct.
    /// Thus, this struct allows for reusing existing (mutable) buffers and still having images
    /// defined by their pixel types.
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::GenericImage;
    /// use ffimage::packed::generic::ImageViewMut;
    ///
    /// let mut mem = vec![0; 3];
    /// let mut buf = ImageViewMut::<Rgb<u8>>::new(&mut mem, 1, 1)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn new(raw: &'a mut [T::T], width: u32, height: u32) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(ImageViewMut {
            raw,
            width,
            height,
            stride,
        })
    }

    #[deprecated(since = "0.8.2", note = "Please use the as_slice() function instead")]
    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    #[deprecated(
        since = "0.8.2",
        note = "Please use the as_mut_slice() function instead"
    )]
    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }

    /// Returns a view to the pixel backing storage
    pub fn as_slice(&self) -> &[T::T] {
        &self.raw
    }

    /// Returns a mutable view to the pixel backing storage
    pub fn as_mut_slice(&mut self) -> &[T::T] {
        &mut self.raw
    }

    /// Returns the length of one pixel row in bytes
    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl<'a, T: Pixel + 'a> GenericImageView<'a> for ImageViewMut<'a, T> {
    impl_GenericImageView!(ImageViewMut);

    type SubImage = SubImageView<'a, Self>;

    fn view(&'a self, x: u32, y: u32, width: u32, height: u32) -> Option<Self::SubImage> {
        SubImageView::new(self, x, y, width, height)
    }
}

impl<'a, T: Pixel + 'a> GenericImage<'a> for ImageViewMut<'a, T> {
    impl_GenericImage!(ImageViewMut);
}

impl<'a, T: Pixel> Index<usize> for ImageViewMut<'a, T> {
    impl_Index!(ImageViewMut);
}

impl<'a, T: Pixel> IndexMut<usize> for ImageViewMut<'a, T> {
    impl_IndexMut!(ImageViewMut);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, ImageViewMut<'a, T>> {
    impl_Iterator!(ImageViewMut);
}

impl<'a, T: Pixel> Iterator for PixelIterMut<'a, ImageViewMut<'a, T>> {
    impl_IteratorMut!(ImageViewMut);
}

impl<'a, T: Pixel> IntoIterator for &'a ImageViewMut<'a, T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, ImageViewMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

impl<'a, T: Pixel> IntoIterator for &'a mut ImageViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = PixelIterMut<'a, ImageViewMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterMut::new(self)
    }
}

#[derive(Debug, Clone)]
/// Image buffer holding the pixel data
pub struct ImageBuffer<T: Pixel> {
    raw: Vec<T::T>,
    width: u32,
    height: u32,
    stride: usize,
}

impl<T: Pixel> ImageBuffer<T> {
    /// Returns an image buffer with pixel accessors
    ///
    /// The backing memory is allocated by this struct. There is no padding added so only the
    /// minimum amount of memory is consumed. In contrast to flat image buffers, fat buffer
    /// structs own their data and can safely be moved between threads.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::GenericImage;
    /// use ffimage::packed::generic::ImageBuffer;
    ///
    /// let mut buf = ImageBuffer::<Rgb<u8>>::new(3, 3);
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();

        ImageBuffer {
            raw: vec![T::T::zero(); height as usize * width as usize * T::len()],
            width,
            height,
            stride,
        }
    }

    /// Returns an image buffer with pixel accessors
    ///
    /// The pixel memory is copied into an allocated buffer owned by this struct.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `raw` - Pixel memory storage owned by the instance
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::GenericImage;
    /// use ffimage::packed::generic::ImageBuffer;
    ///
    /// let mut mem = vec![0; 3];
    /// let mut buf = ImageBuffer::<Rgb<u8>>::from_raw(1, 1, mem)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn from_raw(width: u32, height: u32, raw: Vec<T::T>) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(ImageBuffer {
            raw,
            width,
            height,
            stride,
        })
    }

    #[deprecated(since = "0.8.2", note = "Please use the as_slice() function instead")]
    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    #[deprecated(
        since = "0.8.2",
        note = "Please use the as_mut_slice() function instead"
    )]
    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }

    /// Returns a view to the pixel backing storage
    pub fn as_slice(&self) -> &[T::T] {
        &self.raw
    }

    /// Returns a mutable view to the pixel backing storage
    pub fn as_mut_slice(&mut self) -> &[T::T] {
        &mut self.raw
    }

    /// Returns the pixel backing storage
    pub fn into_vec(self) -> Vec<T::T> {
        self.raw
    }

    /// Returns the length of one pixel row in bytes
    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl<'a, T: Pixel + 'a> GenericImageView<'a> for ImageBuffer<T> {
    impl_GenericImageView!(ImageBuffer);

    type SubImage = SubImageView<'a, Self>;

    fn view(&'a self, x: u32, y: u32, width: u32, height: u32) -> Option<Self::SubImage> {
        SubImageView::new(self, x, y, width, height)
    }
}

impl<'a, T: Pixel + 'a> GenericImage<'a> for ImageBuffer<T> {
    impl_GenericImage!(ImageBuffer);
}

impl<T: Pixel> Index<usize> for ImageBuffer<T> {
    impl_Index!(ImageBuffer);
}

impl<T: Pixel> IndexMut<usize> for ImageBuffer<T> {
    impl_IndexMut!(ImageBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, ImageBuffer<T>> {
    impl_Iterator!(ImageBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIterMut<'a, ImageBuffer<T>> {
    impl_IteratorMut!(ImageBuffer);
}

impl<'a, T: Pixel> IntoIterator for &'a ImageBuffer<T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, ImageBuffer<T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

impl<'a, T: Pixel> IntoIterator for &'a mut ImageBuffer<T> {
    type Item = &'a mut T;
    type IntoIter = PixelIterMut<'a, ImageBuffer<T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterMut::new(self)
    }
}

impl<T: Pixel> Into<Vec<T::T>> for ImageBuffer<T> {
    fn into(self) -> Vec<T::T> {
        self.raw
    }
}

impl<'a, T: Pixel> From<&ImageView<'a, T>> for ImageBuffer<T> {
    fn from(view: &ImageView<'a, T>) -> Self {
        // unwrap() is safe here because the view itself was checked when it was created
        ImageBuffer::from_raw(view.width(), view.height(), view.as_slice().to_vec()).unwrap()
    }
}

impl<'a, T: Pixel> From<&ImageViewMut<'a, T>> for ImageBuffer<T> {
    fn from(view: &ImageViewMut<'a, T>) -> Self {
        // unwrap() is safe here because the view itself was checked when it was created
        ImageBuffer::from_raw(view.width(), view.height(), view.as_slice().to_vec()).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
/// Sub image view into another image
pub struct SubImageView<'a, I: GenericImageView<'a>> {
    parent: &'a I,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<'a, I: GenericImageView<'a>> SubImageView<'a, I> {
    /// Returns a new read-only view into another image.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent image backing the sub image
    /// * `x` - X offset
    /// * `y` - Y offset
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::GenericImageView;
    /// use ffimage::packed::generic::ImageBuffer;
    ///
    /// let mut buf = ImageBuffer::<Rgb<u8>>::new(3, 3);
    /// let sub = buf.view(1, 1, 2, 2);
    /// ```
    pub fn new(parent: &'a I, x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        if x + width > parent.width() || y + height > parent.height() {
            return None;
        }

        Some(SubImageView {
            parent,
            x,
            y,
            width,
            height,
        })
    }
}

impl<'a, I: GenericImageView<'a>> GenericImageView<'a> for SubImageView<'a, I> {
    type T = I::T;
    type SubImage = Self;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn pixel(&self, x: u32, y: u32) -> Option<Self::T> {
        self.parent.pixel(x + self.x, y + self.y)
    }

    fn view(&'a self, x: u32, y: u32, width: u32, height: u32) -> Option<Self::SubImage> {
        SubImageView::new(self.parent, x + self.x, y + self.y, width, height)
    }
}

impl<'a, T: Pixel> Index<usize> for SubImageView<'a, ImageView<'a, T>> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let row = &self.parent[index + self.y as usize];
        &row[self.x as usize..]
    }
}

impl<'a, T: Pixel> Index<usize> for SubImageView<'a, ImageViewMut<'a, T>> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let row = &self.parent[index + self.y as usize];
        &row[self.x as usize..]
    }
}

impl<'a, T: Pixel> Index<usize> for SubImageView<'a, ImageBuffer<T>> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let row = &self.parent[index + self.y as usize];
        &row[self.x as usize..]
    }
}
