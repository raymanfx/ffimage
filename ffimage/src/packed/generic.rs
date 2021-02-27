use core::ops::{Index, IndexMut};

use std::marker::PhantomData;

use crate::core::traits::{GenericImage, GenericImageView, Pixel};

#[derive(Debug, Clone, Copy)]
/// Image view parametrized by its pixel type
pub struct Image<T: Pixel, B> {
    raw: Matrix<T::T, B>,
    width: u32,
    height: u32,

    marker: PhantomData<T>,
}

impl<T, B> Image<T, B>
where
    T: Pixel,
{
    /// Decomposes the image and returns the backing buffer
    pub fn into_buf(self) -> B {
        self.raw.into_buf()
    }

    /// Returns the length of one pixel row in bytes
    pub fn stride(&self) -> usize {
        self.raw.row_stride()
    }
}

impl<'a, T> Image<T, Vec<T::T>>
where
    T: Pixel,
    T::T: Clone,
{
    /// Returns a newly allocated image.
    ///
    /// # Arguments
    ///
    /// * `width` - Number of columns
    /// * `height` - Number of rows
    /// * `value` - Initial value for all channels
    pub fn new(width: u32, height: u32, value: T::T) -> Self {
        let stride = width / T::subpixels() as u32 * T::channels() as u32;
        let raw = Matrix::new(height, stride, value);

        Image {
            raw,
            width,
            height,
            marker: PhantomData::default(),
        }
    }
}

impl<T, B> Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]>,
{
    /// Returns an image view with pixel accessors
    ///
    /// The backing memory storage must have the same element type as the underlying pixel type of
    /// the image.
    ///
    /// # Arguments
    ///
    /// * `buf` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    pub fn from_buf(buf: B, width: u32, height: u32) -> Option<Self> {
        let stride = width / T::subpixels() as u32 * T::channels() as u32;
        let raw = Matrix::from_buf(buf, height, stride)?;

        Some(Image {
            raw,
            width,
            height,
            marker: PhantomData::default(),
        })
    }

    /// Returns an image view with pixel accessors
    ///
    /// The backing memory storage must have the same element type as the underlying pixel type of
    /// the image.
    ///
    /// # Arguments
    ///
    /// * `buf` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `stride` - Length of one pixel row in bytes
    pub fn from_buf_with_stride(buf: B, width: u32, height: u32, stride: usize) -> Option<Self> {
        let raw = Matrix::from_buf_with_stride(buf, height, stride, width * T::channels() as u32)?;

        Some(Image {
            raw,
            width,
            height,
            marker: PhantomData::default(),
        })
    }
}

impl<'a, T, B> AsRef<[T::T]> for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]>,
{
    fn as_ref(&self) -> &[T::T] {
        self.raw.as_ref()
    }
}

impl<'a, T, B> AsMut<[T::T]> for Image<T, B>
where
    T: Pixel,
    B: AsMut<[T::T]>,
{
    fn as_mut(&mut self) -> &mut [T::T] {
        self.raw.as_mut()
    }
}

impl<'a, T, B> GenericImageView for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]>,
{
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
}

impl<'a, T, B> GenericImage for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), ()> {
        if x >= self.width() || y >= self.height() {
            return Err(());
        }

        self[y as usize][x as usize] = *pix;
        Ok(())
    }
}

impl<T, B> Index<usize> for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]>,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let row = &self.raw[index];
        let (head, body, _tail) = unsafe { row.align_to::<T>() };
        assert!(head.is_empty(), "raw data is not aligned");
        assert_eq!(
            body.len(),
            (self.width() / T::subpixels() as u32) as usize,
            "invalid number of row items"
        );

        body
    }
}

impl<T, B> IndexMut<usize> for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let width = self.width();
        let row = &mut self.raw[index];
        let (head, body, _tail) = unsafe { row.align_to_mut::<T>() };
        assert!(head.is_empty(), "raw data is not aligned");
        assert_eq!(
            body.len(),
            (width / T::subpixels() as u32) as usize,
            "invalid number of row items"
        );

        body
    }
}

#[derive(Debug, Clone, Copy)]
/// 2D matrix in row-major order.
pub struct Matrix<T, B> {
    mem: B,
    rows: u32,
    row_stride: usize,
    cols: u32,

    marker: PhantomData<T>,
}

impl<T, B> Matrix<T, B> {
    /// Decomposes the matrix and returns the backing buffer
    pub fn into_buf(self) -> B {
        self.mem
    }

    /// Returns the row count.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Returns the column count.
    pub fn cols(&self) -> u32 {
        self.cols
    }

    /// Returns the length of one row in size of T.
    pub fn row_stride(&self) -> usize {
        self.row_stride
    }
}

impl<T> Matrix<T, Vec<T>>
where
    T: Clone,
{
    /// Returns a newly allocated matrix.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows
    /// * `cols` - Number of columns
    /// * `value` - Initial value for all cells
    pub fn new(rows: u32, cols: u32, value: T) -> Self {
        let mut mem = Vec::new();
        let row_stride = cols as usize;
        let new_len = rows as usize * row_stride;
        mem.resize(new_len, value);

        Matrix {
            mem,
            rows,
            row_stride,
            cols,
            marker: PhantomData::default(),
        }
    }

    /// Returns a view to the pixel backing storage.
    pub fn resize(&mut self, rows: u32, cols: u32, value: T) {
        let row_stride = cols as usize;
        let new_len = rows as usize * row_stride;
        self.mem.resize(new_len, value);
        self.rows = rows;
        self.cols = cols;
        self.row_stride = row_stride;
    }
}

impl<T, B> Matrix<T, B>
where
    B: AsRef<[T]>,
{
    /// Returns a flat matrix backed by an existing buffer.
    ///
    /// In case of an inappropriate buffer length, None is returned.
    ///
    /// # Arguments
    ///
    /// * `buf` - Backing buffer
    /// * `rows` - Number of rows
    /// * `cols` - Number of columns
    pub fn from_buf(buf: B, rows: u32, cols: u32) -> Option<Self> {
        // validate bytes per line
        let row_stride = cols as usize;
        if buf.as_ref().len() < row_stride {
            return None;
        }

        Some(Matrix {
            mem: buf,
            rows,
            row_stride,
            cols,
            marker: PhantomData::default(),
        })
    }

    /// Returns a flat matrix backed by an existing buffer.
    ///
    /// In case of an inappropriate buffer length, None is returned.
    ///
    /// # Arguments
    ///
    /// * `buf` - Backing buffer
    /// * `rows` - Number of rows
    /// * `row_stride` - Length of one row in size of T
    /// * `cols` - Number of columns
    pub fn from_buf_with_stride(buf: B, rows: u32, row_stride: usize, cols: u32) -> Option<Self> {
        // validate bytes per line
        let min_stride = cols as usize;
        if row_stride < min_stride || buf.as_ref().len() < rows as usize * min_stride {
            return None;
        }

        Some(Matrix {
            mem: buf,
            rows,
            row_stride,
            cols,
            marker: PhantomData::default(),
        })
    }
}

impl<T, B> AsRef<[T]> for Matrix<T, B>
where
    B: AsRef<[T]>,
{
    fn as_ref(&self) -> &[T] {
        self.mem.as_ref()
    }
}

impl<T, B> AsMut<[T]> for Matrix<T, B>
where
    B: AsMut<[T]>,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.mem.as_mut()
    }
}

impl<T, B> Index<usize> for Matrix<T, B>
where
    B: AsRef<[T]>,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        // determine the offset in the raw buffer
        let row_stride = self.row_stride();
        let offset = index * row_stride;
        let mem = self.mem.as_ref();
        &mem[offset..offset + row_stride]
    }
}

impl<T, B> IndexMut<usize> for Matrix<T, B>
where
    B: AsRef<[T]> + AsMut<[T]>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // determine the offset in the raw buffer
        let row_stride = self.row_stride();
        let offset = index * row_stride;
        let mem = self.mem.as_mut();
        &mut mem[offset..offset + row_stride]
    }
}
