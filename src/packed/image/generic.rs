use std::marker::PhantomData;
use std::mem;

use crate::core::iter::{PixelIter, PixelIterMut};
use crate::core::traits::{CloneImage, ImageBuffer, ImageView, Pixel, Resize};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_ImageView {
    ($id:ident) => {
        impl<'a, T: Pixel> ImageView for $id<'a, T> {
            type T = T;

            fn width(&self) -> u32 {
                self.width
            }

            fn height(&self) -> u32 {
                self.height
            }

            fn stride(&self) -> usize {
                (self.width() * T::channels() as u32) as usize * mem::size_of::<T::T>()
                    + self.row_padding
            }

            fn get_pixel(&self, x: u32, y: u32) -> Option<Self::T> {
                if x >= self.width() || y >= self.height() {
                    return None;
                }

                // determine the offset in the raw buffer
                let stride_elems = self.stride() / mem::size_of::<T::T>();
                let off: usize = y as usize * stride_elems + x as usize * T::channels() as usize;
                let slice = &self.raw[off..off + T::channels() as usize];
                match Self::T::try_from(slice) {
                    Ok(pix) => Some(pix),
                    Err(_) => None,
                }
            }
        }
    };
}

macro_rules! impl_ImageBuffer {
    ($id:ident) => {
        impl<'a, T: Pixel> ImageBuffer for $id<'a, T> {
            fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), ()> {
                if x >= self.width() || y >= self.height() {
                    return Err(());
                }

                // determine the offset in the raw buffer
                let stride_elems = self.stride() / mem::size_of::<T::T>();
                let off: usize = y as usize * stride_elems + x as usize * T::channels() as usize;
                let slice = &mut self.raw[off..off + T::channels() as usize];
                for i in 0..slice.len() {
                    // i can never be out of bounds because the pixel is strongly typed
                    slice[i] = pix.at(i);
                }

                Ok(())
            }
        }
    };
}

macro_rules! impl_CloneImage {
    ($id:ident) => {
        impl<'a, T: Pixel> CloneImage for $id<'a, T> {
            type Output = GenericBuffer<'a, T>;

            fn clone_into(&self, output: &mut Self::Output) {
                output.resize(self.width, self.height);
                // copy data without padding
                for i in (0..self.height) {
                    let src = self.pixel_row(i).unwrap();
                    let dst = output.pixel_row_mut(i).unwrap();
                    dst.copy_from_slice(src);
                }
            }

            fn clone(&self) -> Self::Output {
                let mut output = Self::Output::new(self.width, self.height);
                self.clone_into(&mut output);
                output
            }
        }
    };
}

macro_rules! impl_Resize {
    ($id:ident) => {
        impl<'a, T: Pixel> Resize for $id<'a, T> {
            fn resize(&mut self, width: u32, height: u32) {
                self.width = width;
                self.height = height;
                self.row_padding = 0;
                self.raw.resize(
                    (width * height * T::channels() as u32) as usize,
                    T::T::default(),
                );
            }
        }
    };
}

macro_rules! impl_AccessPixel {
    ($id:ident) => {
        impl<'a, T: Pixel> AccessPixel for $id<'a, T> {
            type PixelType = T;

            fn pixel_row(&self, y: u32) -> Option<&[Self::PixelType]> {
                if y >= self.height() {
                    return None;
                }

                // determine the offset in the raw buffer
                let stride_elems = self.stride() / mem::size_of::<T::T>();
                let off: usize = y as usize * stride_elems;
                let slice = &self.raw[off..off + stride_elems];
                let (head, body, _tail) = unsafe { slice.align_to::<T>() };
                assert!(head.is_empty(), "raw data is not aligned");
                assert_eq!(
                    body.len(),
                    self.width() as usize,
                    "invalid number of row items"
                );

                Some(&body)
            }

            fn pixel(&self, x: u32, y: u32) -> Option<&Self::PixelType> {
                if x >= self.width() || y >= self.height() {
                    return None;
                }

                let row = self.pixel_row(y)?;
                Some(&row[x as usize])
            }
        }
    };
}

macro_rules! impl_AccessPixelMut {
    ($id:ident) => {
        impl<'a, T: Pixel> AccessPixelMut for $id<'a, T> {
            type PixelType = T;

            fn pixel_row_mut(&mut self, y: u32) -> Option<&mut [Self::PixelType]> {
                let width = self.width();
                if y >= self.height() {
                    return None;
                }

                // determine the offset in the raw buffer
                let stride_elems = self.stride() / mem::size_of::<T::T>();
                let off: usize = y as usize * stride_elems;
                let slice = &mut self.raw[off..off + stride_elems];
                let (head, body, _tail) = unsafe { slice.align_to_mut::<T>() };
                assert!(head.is_empty(), "raw data is not aligned");
                assert_eq!(body.len(), width as usize, "invalid number of row items");

                Some(&mut body[..])
            }

            fn pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::PixelType> {
                if x >= self.width() || y >= self.height() {
                    return None;
                }

                let row = self.pixel_row_mut(y)?;
                Some(&mut row[x as usize])
            }
        }
    };
}

macro_rules! impl_IntoIterator {
    ($id:ident) => {
        impl<'a, T: Pixel> Iterator for PixelIter<'a, $id<'a, T>> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.x >= self.width {
                    self.x = 0;
                    self.y += 1;
                }

                if self.y >= self.height {
                    return None;
                }

                let pixel = self.img.pixel(self.x, self.y);
                self.x += 1;

                pixel
            }
        }

        impl<'a, T: Pixel> IntoIterator for &'a $id<'a, T> {
            type Item = &'a T;
            type IntoIter = PixelIter<'a, $id<'a, T>>;

            fn into_iter(self) -> PixelIter<'a, $id<'a, T>> {
                PixelIter::new(self)
            }
        }
    };
}

// iterators handing out mutable references are not allowed by safe rust as explained here:
// https://stackoverflow.com/a/27641876/11423991
macro_rules! impl_IntoIteratorMut {
    ($id:ident) => {
        impl<'a, T: Pixel> Iterator for PixelIterMut<'a, $id<'a, T>> {
            type Item = &'a mut T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.x >= self.width {
                    self.x = 0;
                    self.y += 1;
                }

                if self.y >= self.height {
                    return None;
                }

                let pixel = self.img.pixel_mut(self.x, self.y);
                self.x += 1;

                // This is safe because...
                // (from http://stackoverflow.com/questions/25730586):
                // The Rust compiler does not know that when you ask a mutable iterator for the
                // next element, that you get a different reference every time and never the same
                // reference twice. Of course, we know that such an iterator won't give you the
                // same reference twice.
                unsafe { mem::transmute(pixel) }
            }
        }

        impl<'a, T: Pixel> IntoIterator for &'a mut $id<'a, T> {
            type Item = &'a mut T;
            type IntoIter = PixelIterMut<'a, $id<'a, T>>;

            fn into_iter(self) -> PixelIterMut<'a, $id<'a, T>> {
                PixelIterMut::new(self)
            }
        }
    };
}

/// Image view parametrized by its pixel type
pub struct GenericView<'a, T: Pixel> {
    raw: &'a [T::T],
    width: u32,
    height: u32,
    row_padding: usize,
}

impl<'a, T: Pixel> GenericView<'a, T> {
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
    /// use ffimage::packed::GenericImageView;
    ///
    /// let mem = vec![0; 3];
    /// let view = GenericImageView::<Rgb<u8>>::new(&mem, 1, 1).expect("Memory region too small");
    /// ```
    pub fn new(raw: &'a [T::T], width: u32, height: u32) -> Option<Self> {
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let raw_len = raw.len() * mem::size_of::<T::T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(GenericView {
                raw,
                width,
                height,
                row_padding: 0,
            })
        }
    }

    /// Returns an image view with pixel accessors
    ///
    /// The backing memory storage must have the same element type as the underlying pixel type of
    /// the image. This constructor takes an additional stride for strided image buffers.
    /// The stride must be a multiple of the size of the internal backing type T of the pixel.
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw memory region to interpret as typed image
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::packed::GenericImageView;
    ///
    /// let mem = vec![0; 14];
    /// let view = GenericImageView::<Rgb<u8>>::with_stride(&mem, 2, 2, 7 /* one byte padding */)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride(raw: &'a [T::T], width: u32, height: u32, stride: usize) -> Option<Self> {
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let raw_len = raw.len() * mem::size_of::<T::T>();

        if raw_len < height as usize * stride || stride % mem::size_of::<T::T>() != 0 {
            None
        } else {
            Some(GenericView {
                raw,
                width,
                height,
                row_padding: stride - min_stride,
            })
        }
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }
}

impl_ImageView!(GenericView);
impl_AccessPixel!(GenericView);
impl_IntoIterator!(GenericView);
impl_CloneImage!(GenericView);

pub struct GenericFlatBuffer<'a, T: Pixel> {
    raw: &'a mut [T::T],
    width: u32,
    height: u32,
    row_padding: usize,
}

impl<'a, T: Pixel> GenericFlatBuffer<'a, T> {
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
    /// use ffimage::core::ImageBuffer;
    /// use ffimage::packed::GenericImageFlatBuffer;
    ///
    /// let mut mem = vec![0; 3];
    /// let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::new(&mut mem, 1, 1)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn new(raw: &'a mut [T::T], width: u32, height: u32) -> Option<Self> {
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let raw_len = raw.len() * mem::size_of::<T::T>();

        if raw_len < height as usize * min_stride {
            None
        } else {
            Some(GenericFlatBuffer {
                raw,
                width,
                height,
                row_padding: 0,
            })
        }
    }

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
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::ImageBuffer;
    /// use ffimage::packed::GenericImageFlatBuffer;
    ///
    /// let mut mem = vec![0; 4];
    /// let mut buf = GenericImageFlatBuffer::<Rgb<u8>>::with_stride(&mut mem, 1, 1, 4)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn with_stride(
        raw: &'a mut [T::T],
        width: u32,
        height: u32,
        stride: usize,
    ) -> Option<Self> {
        let min_stride = width as usize * T::channels() as usize * mem::size_of::<T::T>();
        let raw_len = raw.len() * mem::size_of::<T::T>();

        if raw_len < height as usize * stride || stride % mem::size_of::<T::T>() != 0 {
            None
        } else {
            Some(GenericFlatBuffer {
                raw,
                width,
                height,
                row_padding: stride - min_stride,
            })
        }
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }
}

impl_ImageView!(GenericFlatBuffer);
impl_ImageBuffer!(GenericFlatBuffer);
impl_CloneImage!(GenericFlatBuffer);
impl_AccessPixel!(GenericFlatBuffer);
impl_AccessPixelMut!(GenericFlatBuffer);
impl_IntoIterator!(GenericFlatBuffer);
impl_IntoIteratorMut!(GenericFlatBuffer);

pub struct GenericBuffer<'a, T: Pixel> {
    raw: Vec<T::T>,
    width: u32,
    height: u32,
    row_padding: usize,

    phantom: PhantomData<&'a ()>,
}

impl<'a, T: Pixel> GenericBuffer<'a, T> {
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
    /// use ffimage::core::ImageBuffer;
    /// use ffimage::packed::GenericImageBuffer;
    ///
    /// let mut buf = GenericImageBuffer::<Rgb<u8>>::new(3, 3);
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        GenericBuffer {
            raw: vec![T::T::default(); height as usize * width as usize * T::len()],
            width,
            height,
            row_padding: 0,
            phantom: PhantomData,
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
    /// * `raw` - Pixel memory storage to copy
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::ImageBuffer;
    /// use ffimage::packed::GenericImageBuffer;
    ///
    /// let mem = vec![0; 3];
    /// let mut buf = GenericImageBuffer::<Rgb<u8>>::with_raw(1, 1, &mem)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn with_raw(width: u32, height: u32, raw: &[T::T]) -> Option<Self> {
        let stride = width as usize * T::len();
        if raw.len() < height as usize * stride {
            None
        } else {
            Some(GenericBuffer {
                raw: raw.to_vec(),
                width,
                height,
                row_padding: 0,
                phantom: PhantomData,
            })
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
    /// * `raw` - Pixel memory storage to copy
    /// * `stride` - Length of a pixel row in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::color::rgb::*;
    /// use ffimage::core::ImageBuffer;
    /// use ffimage::packed::GenericImageBuffer;
    ///
    /// let mem = vec![0; 3];
    /// let mut buf = GenericImageBuffer::<Rgb<u8>>::with_raw_stride(1, 1, &mem, 3)
    ///     .expect("Memory region too small");
    /// let pix = Rgb::<u8>::new([255, 255, 255]);
    /// buf.set_pixel(0, 0, &pix).unwrap();
    /// ```
    pub fn with_raw_stride(width: u32, height: u32, raw: &[T::T], stride: usize) -> Option<Self> {
        if raw.len() < height as usize * stride {
            None
        } else {
            Some(GenericBuffer {
                raw: raw.to_vec(),
                width,
                height,
                row_padding: 0,
                phantom: PhantomData,
            })
        }
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }
}

impl_ImageView!(GenericBuffer);
impl_ImageBuffer!(GenericBuffer);
impl_CloneImage!(GenericBuffer);
impl_Resize!(GenericBuffer);
impl_AccessPixel!(GenericBuffer);
impl_AccessPixelMut!(GenericBuffer);
impl_IntoIterator!(GenericBuffer);
impl_IntoIteratorMut!(GenericBuffer);
