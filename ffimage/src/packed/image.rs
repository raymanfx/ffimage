use core::ops::{Index, IndexMut};

use std::marker::PhantomData;

use crate::error::Error;
use crate::packed::Matrix;
use crate::traits::{GenericImage, GenericImageView, Pixel};

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

impl<T> Image<T, Vec<T::T>>
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

impl<T, B> AsRef<[T::T]> for Image<T, B>
where
    T: Pixel,
    B: AsRef<[T::T]>,
{
    fn as_ref(&self) -> &[T::T] {
        self.raw.as_ref()
    }
}

impl<T, B> AsMut<[T::T]> for Image<T, B>
where
    T: Pixel,
    B: AsMut<[T::T]>,
{
    fn as_mut(&mut self) -> &mut [T::T] {
        self.raw.as_mut()
    }
}

impl<T, B> GenericImageView for Image<T, B>
where
    T: Pixel + Copy,
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

impl<T, B> GenericImage for Image<T, B>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), Error> {
        if x >= self.width() || y >= self.height() {
            return Err(Error::OutOfBounds);
        }

        self[y as usize][x as usize] = *pix;
        Ok(())
    }
}

impl<T, B> Index<usize> for Image<T, B>
where
    T: Pixel + Copy,
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
    T: Pixel + Copy,
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
