use std::mem;

use num_traits::identities::Zero;

use crate::core::iter::{PixelIter, PixelIterMut};
use crate::core::traits::{CloneImage, ImageBuffer, ImageView, Pixel};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

macro_rules! impl_ImageView {
    ($id:ident) => {
        type T = T;

        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn stride(&self) -> usize {
            self.stride
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
    };
}

macro_rules! impl_ImageBuffer {
    ($id:ident) => {
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
    };
}

macro_rules! impl_CloneImage {
    ($id:ident) => {
        type Output = GenericBuffer<T>;

        fn clone_into(&self, output: &mut Self::Output) {
            *output = Self::Output::new(self.width(), self.height());
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
    };
}

macro_rules! impl_AccessPixel {
    ($id:ident) => {
        type PixelType = T;

        fn pixel_row(&self, y: u32) -> Option<&[Self::PixelType]> {
            if y >= self.height() {
                return None;
            }

            // determine the offset in the raw buffer
            let pixels_per_row = self.width() / T::subpixels() as u32;
            let stride_elems = self.stride() / mem::size_of::<T::T>();
            let off: usize = y as usize * stride_elems;
            let slice = &self.raw[off..off + stride_elems];
            let (head, body, _tail) = unsafe { slice.align_to::<T>() };
            assert!(head.is_empty(), "raw data is not aligned");
            assert_eq!(
                body.len(),
                pixels_per_row as usize,
                "invalid number of row items"
            );

            Some(body)
        }

        fn pixel(&self, x: u32, y: u32) -> Option<&Self::PixelType> {
            if x >= self.width() || y >= self.height() {
                return None;
            }

            let row = self.pixel_row(y)?;
            let x = x / T::subpixels() as u32;
            Some(&row[x as usize])
        }
    };
}

macro_rules! impl_AccessPixelMut {
    ($id:ident) => {
        type PixelType = T;

        fn pixel_row_mut(&mut self, y: u32) -> Option<&mut [Self::PixelType]> {
            if y >= self.height() {
                return None;
            }

            // determine the offset in the raw buffer
            let pixels_per_row = self.width() / T::subpixels() as u32;
            let stride_elems = self.stride() / mem::size_of::<T::T>();
            let off: usize = y as usize * stride_elems;
            let slice = &mut self.raw[off..off + stride_elems];
            let (head, body, _tail) = unsafe { slice.align_to_mut::<T>() };
            assert!(head.is_empty(), "raw data is not aligned");
            assert_eq!(
                body.len(),
                pixels_per_row as usize,
                "invalid number of row items"
            );

            Some(body)
        }

        fn pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::PixelType> {
            if x >= self.width() || y >= self.height() {
                return None;
            }

            let row = self.pixel_row_mut(y)?;
            let x = x / T::subpixels() as u32;
            Some(&mut row[x as usize])
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

            let pixel = self.img.pixel(self.x, self.y);
            self.x += 1;

            pixel
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
    };
}

/// Image view parametrized by its pixel type
pub struct GenericView<'a, T: Pixel> {
    raw: &'a [T::T],
    width: u32,
    height: u32,
    stride: usize,
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
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let pixels_per_row = width / T::subpixels() as u32;
        let min_stride = pixels_per_row as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(GenericView {
            raw,
            width,
            height,
            stride,
        })
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }
}

impl<'a, T: Pixel> ImageView for GenericView<'a, T> {
    impl_ImageView!(GenericView);
}

impl<'a, T: Pixel> CloneImage for GenericView<'a, T> {
    impl_CloneImage!(GenericView);
}

impl<'a, T: Pixel> AccessPixel for GenericView<'a, T> {
    impl_AccessPixel!(GenericView);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, GenericView<'a, T>> {
    impl_Iterator!(GenericView);
}

impl<'a, T: Pixel> IntoIterator for &'a GenericView<'a, T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, GenericView<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

pub struct GenericFlatBuffer<'a, T: Pixel> {
    raw: &'a mut [T::T],
    width: u32,
    height: u32,
    stride: usize,
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
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let pixels_per_row = width / T::subpixels() as u32;
        let min_stride = pixels_per_row as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(GenericFlatBuffer {
            raw,
            width,
            height,
            stride,
        })
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }
}

impl<'a, T: Pixel> ImageView for GenericFlatBuffer<'a, T> {
    impl_ImageView!(GenericFlatBuffer);
}

impl<'a, T: Pixel> ImageBuffer for GenericFlatBuffer<'a, T> {
    impl_ImageBuffer!(GenericFlatBuffer);
}

impl<'a, T: Pixel> CloneImage for GenericFlatBuffer<'a, T> {
    impl_CloneImage!(GenericFlatBuffer);
}

impl<'a, T: Pixel> AccessPixel for GenericFlatBuffer<'a, T> {
    impl_AccessPixel!(GenericFlatBuffer);
}

impl<'a, T: Pixel> AccessPixelMut for GenericFlatBuffer<'a, T> {
    impl_AccessPixelMut!(GenericFlatBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, GenericFlatBuffer<'a, T>> {
    impl_Iterator!(GenericFlatBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIterMut<'a, GenericFlatBuffer<'a, T>> {
    impl_IteratorMut!(GenericFlatBuffer);
}

impl<'a, T: Pixel> IntoIterator for &'a GenericFlatBuffer<'a, T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, GenericFlatBuffer<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

impl<'a, T: Pixel> IntoIterator for &'a mut GenericFlatBuffer<'a, T> {
    type Item = &'a mut T;
    type IntoIter = PixelIterMut<'a, GenericFlatBuffer<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterMut::new(self)
    }
}

pub struct GenericBuffer<T: Pixel> {
    raw: Vec<T::T>,
    width: u32,
    height: u32,
    stride: usize,
}

impl<T: Pixel> GenericBuffer<T> {
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
        let pixels_per_row = width / T::subpixels() as u32;
        let stride = pixels_per_row as usize * T::channels() as usize * mem::size_of::<T::T>();

        GenericBuffer {
            raw: vec![T::T::zero(); height as usize * pixels_per_row as usize * T::len()],
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
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let pixels_per_row = width / T::subpixels() as u32;
        let min_stride = pixels_per_row as usize * T::channels() as usize * mem::size_of::<T::T>();
        let stride = raw.len() * mem::size_of::<T::T>() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(GenericBuffer {
            raw: raw.to_vec(),
            width,
            height,
            stride,
        })
    }

    pub fn raw(&self) -> &[T::T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T::T] {
        &mut self.raw
    }
}

impl<T: Pixel> ImageView for GenericBuffer<T> {
    impl_ImageView!(GenericBuffer);
}

impl<T: Pixel> ImageBuffer for GenericBuffer<T> {
    impl_ImageBuffer!(GenericBuffer);
}

impl<T: Pixel> CloneImage for GenericBuffer<T> {
    impl_CloneImage!(GenericBuffer);
}

impl<T: Pixel> AccessPixel for GenericBuffer<T> {
    impl_AccessPixel!(GenericBuffer);
}

impl<T: Pixel> AccessPixelMut for GenericBuffer<T> {
    impl_AccessPixelMut!(GenericBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIter<'a, GenericBuffer<T>> {
    impl_Iterator!(GenericBuffer);
}

impl<'a, T: Pixel> Iterator for PixelIterMut<'a, GenericBuffer<T>> {
    impl_IteratorMut!(GenericBuffer);
}

impl<'a, T: Pixel> IntoIterator for &'a GenericBuffer<T> {
    type Item = &'a T;
    type IntoIter = PixelIter<'a, GenericBuffer<T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(self)
    }
}

impl<'a, T: Pixel> IntoIterator for &'a mut GenericBuffer<T> {
    type Item = &'a mut T;
    type IntoIter = PixelIterMut<'a, GenericBuffer<T>>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterMut::new(self)
    }
}
