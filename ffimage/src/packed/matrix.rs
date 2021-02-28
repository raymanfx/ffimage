use core::ops::{Index, IndexMut};

use std::marker::PhantomData;

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
