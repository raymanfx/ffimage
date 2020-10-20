use std::any::TypeId;
use std::convert::TryFrom;
use std::mem;

use crate::core::traits::Pixel;
use crate::packed::generic::ImageView as GenericImageView;

#[derive(Debug, Clone, Copy)]
/// Runtime storage type
pub enum StorageType {
    /* integer types */
    U8 = 1,
    U16 = 2,
}

#[derive(Debug, Clone, Copy)]
/// Runtime memory view
pub enum MemoryView<'a> {
    U8(&'a [u8]),
    U16(&'a [u16]),
}

impl<'a> MemoryView<'a> {
    /// Returns the slice representation of a memory view
    ///
    /// It is ensured that only the proper type representation can be cast from the underlying
    /// view. If, for example, you were to call the method on a U16 view and try to get a [u8]
    /// slice reference, the function would return None instead.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use ffimage::packed::dynamic::MemoryView;
    ///
    /// let mem: Vec<u8> = vec![0; 12];
    /// let view = MemoryView::try_from(&mem[..])
    ///     .expect("Failed to wrap memory region");
    ///
    /// let slice: &[u8] = view.as_slice()
    ///     .expect("Failed to cast memory view");
    /// ```
    pub fn as_slice<T: 'static>(&self) -> Option<&[T]> {
        match self {
            MemoryView::U8(view) => {
                if TypeId::of::<T>() == TypeId::of::<u8>() {
                    let mem: &[u8] = view;
                    unsafe { Some(&*(mem as *const [u8] as *const [T])) }
                } else {
                    None
                }
            }
            MemoryView::U16(view) => {
                if TypeId::of::<T>() == TypeId::of::<u16>() {
                    let mem: &[u16] = view;
                    unsafe { Some(&*(mem as *const [u16] as *const [T])) }
                } else {
                    None
                }
            }
        }
    }

    /// Returns the number of elements in the view
    pub fn len(&self) -> usize {
        match &self {
            MemoryView::U8(view) => view.len(),
            MemoryView::U16(view) => view.len(),
        }
    }

    /// Returns true if there are no elements in the view
    pub fn is_empty(&self) -> bool {
        match &self {
            MemoryView::U8(view) => view.is_empty(),
            MemoryView::U16(view) => view.is_empty(),
        }
    }
}

impl<'a, T: 'static> TryFrom<&[T]> for MemoryView<'a> {
    type Error = ();

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        unsafe {
            if TypeId::of::<T>() == TypeId::of::<u8>() {
                let mem = &*(slice as *const [T] as *const [u8]);
                Ok(MemoryView::U8(mem))
            } else if TypeId::of::<T>() == TypeId::of::<u16>() {
                let mem = &*(slice as *const [T] as *const [u16]);
                Ok(MemoryView::U16(mem))
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone)]
/// Runtime memory buffer
pub enum MemoryBuffer {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

impl MemoryBuffer {
    /// Returns the slice representation of a memory buffer
    ///
    /// It is ensured that only the proper type representation can be cast from the underlying
    /// buffer. If, for example, you were to call the method on a U16 buffer and try to get a [u8]
    /// slice reference, the function would return None instead.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use ffimage::packed::dynamic::{MemoryBuffer, StorageType};
    ///
    /// let mut mem: Vec<u8> = vec![0; 12];
    /// let buf = MemoryBuffer::try_from(mem)
    ///     .expect("Failed to wrap memory region");
    ///
    /// let slice: &[u8] = buf.as_slice()
    ///     .expect("Failed to cast memory buffer");
    /// ```
    pub fn as_slice<T: 'static>(&self) -> Option<&[T]> {
        match self {
            MemoryBuffer::U8(buf) => {
                if TypeId::of::<T>() == TypeId::of::<u8>() {
                    let mem: &[u8] = &buf[..];
                    unsafe { Some(&*(mem as *const [u8] as *const [T])) }
                } else {
                    None
                }
            }
            MemoryBuffer::U16(buf) => {
                if TypeId::of::<T>() == TypeId::of::<u16>() {
                    let mem: &[u16] = &buf[..];
                    unsafe { Some(&*(mem as *const [u16] as *const [T])) }
                } else {
                    None
                }
            }
        }
    }

    /// Returns the mutable slice representation of a memory buffer
    ///
    /// It is ensured that only the proper type representation can be cast from the underlying
    /// buffer. If, for example, you were to call the method on a U16 buffer and try to get a [u8]
    /// slice reference, the function would return None instead.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use ffimage::packed::dynamic::{MemoryBuffer, StorageType};
    ///
    /// let mut mem: Vec<u8> = vec![0; 12];
    /// let mut buf = MemoryBuffer::try_from(mem)
    ///     .expect("Failed to wrap memory region");
    ///
    /// let slice: &mut [u8] = buf.as_mut_slice()
    ///     .expect("Failed to cast memory buffer");
    /// ```
    pub fn as_mut_slice<T: 'static>(&mut self) -> Option<&mut [T]> {
        match self {
            MemoryBuffer::U8(buf) => {
                if TypeId::of::<T>() == TypeId::of::<u8>() {
                    let mem: &mut [u8] = &mut buf[..];
                    unsafe { Some(&mut *(mem as *mut [u8] as *mut [T])) }
                } else {
                    None
                }
            }
            MemoryBuffer::U16(buf) => {
                if TypeId::of::<T>() == TypeId::of::<u16>() {
                    let mem: &mut [u16] = &mut buf[..];
                    unsafe { Some(&mut *(mem as *mut [u16] as *mut [T])) }
                } else {
                    None
                }
            }
        }
    }

    /// Returns the number of elements in the buffer
    pub fn len(&self) -> usize {
        match &self {
            MemoryBuffer::U8(buf) => buf.len(),
            MemoryBuffer::U16(buf) => buf.len(),
        }
    }

    /// Returns true if there are no elements in the buffer
    pub fn is_empty(&self) -> bool {
        match &self {
            MemoryBuffer::U8(buf) => buf.is_empty(),
            MemoryBuffer::U16(buf) => buf.is_empty(),
        }
    }
}

impl<'a> From<&MemoryView<'a>> for MemoryBuffer {
    fn from(view: &MemoryView<'a>) -> Self {
        match view {
            MemoryView::U8(raw) => MemoryBuffer::U8(raw.to_vec()),
            MemoryView::U16(raw) => MemoryBuffer::U16(raw.to_vec()),
        }
    }
}

impl<T: 'static> TryFrom<Vec<T>> for MemoryBuffer {
    type Error = ();

    fn try_from(raw: Vec<T>) -> Result<Self, Self::Error> {
        let mut raw = raw;
        unsafe {
            if TypeId::of::<T>() == TypeId::of::<u8>() {
                let mem = raw.as_mut_ptr() as *mut u8;
                let len = raw.len();
                let cap = raw.capacity();
                mem::forget(raw);
                Ok(MemoryBuffer::U8(Vec::from_raw_parts(mem, len, cap)))
            } else if TypeId::of::<T>() == TypeId::of::<u16>() {
                let mem = raw.as_mut_ptr() as *mut u16;
                let len = raw.len();
                let cap = raw.capacity();
                mem::forget(raw);
                Ok(MemoryBuffer::U16(Vec::from_raw_parts(mem, len, cap)))
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Image view parametrized by its pixel type
pub struct ImageView<'a> {
    raw: MemoryView<'a>,
    width: u32,
    height: u32,
    stride: usize,
}

impl<'a> ImageView<'a> {
    /// Returns an image view with unknown pixel type
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
    /// use ffimage::packed::dynamic::ImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = ImageView::new(&mem, 2, 2);
    /// ```
    pub fn new<T: 'static>(raw: &'a [T], width: u32, height: u32) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * mem::size_of::<T>();
        let stride = raw.len() / height as usize;
        if stride < min_stride {
            return None;
        }

        let mem = MemoryView::try_from(raw);
        match mem {
            Ok(raw) => Some(ImageView {
                raw,
                width,
                height,
                stride,
            }),
            Err(_) => None,
        }
    }

    /// Returns an image view with unknown pixel type
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
    /// use ffimage::packed::dynamic::ImageView;
    ///
    /// let mem: Vec<u8> = vec![0; 12];
    /// let view = ImageView::with_stride(&mem, 2, 2, 6)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride<T: 'static>(
        raw: &'a [T],
        width: u32,
        height: u32,
        stride: usize,
    ) -> Option<Self> {
        let len = height as usize * stride;

        if stride > 0 && raw.len() != len {
            return None;
        }

        let mem = MemoryView::try_from(raw);
        match mem {
            Ok(raw) => Some(ImageView {
                raw,
                width,
                height,
                stride,
            }),
            Err(_) => None,
        }
    }

    /// Returns the raw memory
    pub fn raw(&self) -> &MemoryView {
        &self.raw
    }

    /// Returns the width in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the length of one image row in bytes
    pub fn stride(&self) -> usize {
        self.stride
    }
}

#[derive(Debug, Clone)]
/// Image buffer parametrized by its pixel type
pub struct ImageBuffer {
    raw: MemoryBuffer,
    width: u32,
    height: u32,
    stride: usize,
}

impl ImageBuffer {
    /// Returns an empty image buffer
    ///
    /// # Arguments
    ///
    /// * `typ` - Storage type
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::dynamic::{ImageBuffer, StorageType};
    ///
    /// let buf = ImageBuffer::empty(StorageType::U8);
    /// ```
    pub fn empty(typ: StorageType) -> Self {
        let raw: MemoryBuffer;
        match typ {
            StorageType::U8 => {
                raw = MemoryBuffer::U8(Vec::new());
            }
            StorageType::U16 => {
                raw = MemoryBuffer::U16(Vec::new());
            }
        }

        ImageBuffer {
            raw,
            width: 0,
            height: 0,
            stride: 0,
        }
    }

    /// Returns an image buffer with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `channels` - Number of channels
    /// * `typ` - Storage type
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::dynamic::{ImageBuffer, StorageType};
    ///
    /// let buf = ImageBuffer::new(2, 2, 3, StorageType::U8);
    /// ```
    pub fn new(width: u32, height: u32, channels: u32, typ: StorageType) -> Self {
        let elems = (height * width * channels) as usize * typ as usize;
        let stride = width as usize * channels as usize * typ as usize;
        let raw: MemoryBuffer;
        match typ {
            StorageType::U8 => {
                let mut buf = Vec::new();
                buf.resize(elems, 0);
                raw = MemoryBuffer::U8(buf);
            }
            StorageType::U16 => {
                let mut buf = Vec::new();
                buf.resize(elems, 0);
                raw = MemoryBuffer::U16(buf);
            }
        }

        ImageBuffer {
            raw,
            width,
            height,
            stride,
        }
    }

    /// Returns an image buffer with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `typ` - Storage type
    /// * `raw` - Raw memory region owned by the instance
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::dynamic::ImageBuffer;
    ///
    /// let mem: Vec<u8> = vec![0; 12];
    /// let buf = ImageBuffer::from_raw(2, 2, mem);
    /// ```
    pub fn from_raw<T: 'static>(width: u32, height: u32, raw: Vec<T>) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize * mem::size_of::<T>();
        let stride = raw.len() / height as usize;
        if stride < min_stride {
            return None;
        }

        let mem = MemoryBuffer::try_from(raw);
        match mem {
            Ok(raw) => Some(ImageBuffer {
                raw,
                width,
                height,
                stride,
            }),
            Err(_) => None,
        }
    }

    /// Returns the raw memory
    pub fn raw(&self) -> &MemoryBuffer {
        &self.raw
    }

    /// Returns the raw memory
    pub fn raw_mut(&mut self) -> &mut MemoryBuffer {
        &mut self.raw
    }

    /// Returns the width in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the length of one image row in bytes
    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn resize(&mut self, width: u32, height: u32, channels: u32) {
        self.width = width;
        self.height = height;

        match &mut self.raw {
            MemoryBuffer::U8(buf) => {
                self.stride = width as usize * channels as usize;
                buf.resize(self.height as usize * self.stride, 0);
            }
            MemoryBuffer::U16(buf) => {
                self.stride = width as usize * channels as usize * 2 /* u16 */;
                buf.resize(self.height as usize * self.stride, 0);
            }
        }
    }

    pub fn as_view(&self) -> ImageView {
        match &self.raw {
            MemoryBuffer::U8(data) => {
                let view = ImageView::new(data, self.width, self.height);
                view.unwrap()
            }
            MemoryBuffer::U16(data) => {
                let view = ImageView::new(data.as_slice(), self.width, self.height);
                view.unwrap()
            }
        }
    }
}

impl<'a> From<&ImageView<'a>> for ImageBuffer {
    fn from(input: &ImageView<'a>) -> Self {
        ImageBuffer {
            raw: MemoryBuffer::from(&input.raw),
            width: input.width,
            height: input.height,
            stride: input.stride,
        }
    }
}

impl<'a, T> TryFrom<&ImageView<'a>> for GenericImageView<'a, T>
where
    T: Pixel<T = u8>,
{
    type Error = ();

    fn try_from(input: &ImageView<'a>) -> Result<Self, Self::Error> {
        let mem: &'a [u8];
        match input.raw {
            MemoryView::U8(view) => mem = view,
            _ => return Err(()),
        }

        let view = GenericImageView::<T>::new(mem, input.width, input.height);
        match view {
            Some(view) => Ok(view),
            None => Err(()),
        }
    }
}
