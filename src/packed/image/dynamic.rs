use std::any::TypeId;
use std::convert::TryFrom;

use crate::core::traits::Pixel;
use crate::packed::image::generic::GenericView;

#[derive(Clone, Copy)]
/// Runtime storage type
pub enum StorageType {
    /* integer types */
    U8 = 1,
    U16 = 2,
}

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
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::new(&mem, 2, 2)
    ///     .expect("Memory region too small");
    ///
    /// let slice: &[u8] = view.raw.as_slice()
    ///     .expect("Failed to cast memory view");
    /// ```
    pub fn as_slice<T: 'static>(&self) -> Option<&[T]> {
        match &self {
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
    /// use ffimage::packed::{DynamicImageBuffer, DynamicStorageType};
    ///
    /// let buf = DynamicImageBuffer::new(2, 2, 3, DynamicStorageType::U8);
    ///
    /// let slice: &[u8] = buf.raw.as_slice()
    ///     .expect("Failed to cast memory buffer");
    /// ```
    pub fn as_slice<T: 'static>(&self) -> Option<&[T]> {
        match &self {
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

/// Image view parametrized by its pixel type
pub struct DynamicView<'a> {
    pub raw: MemoryView<'a>,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub typ: StorageType,
}

impl<'a> DynamicView<'a> {
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
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::new(&mem, 2, 2);
    /// ```
    pub fn new(raw: &'a [u8], width: u32, height: u32) -> Option<Self> {
        // require the same amount of elements per row
        if raw.len() % height as usize != 0 {
            return None;
        }

        // validate bytes per line
        let min_stride = width as usize;
        let stride = raw.len() / height as usize;
        if stride < min_stride {
            return None;
        }

        Some(DynamicView {
            raw: MemoryView::U8(raw),
            width,
            height,
            stride,
            typ: StorageType::U8,
        })
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
    /// use ffimage::packed::DynamicImageView;
    ///
    /// let mem = vec![0; 12];
    /// let view = DynamicImageView::with_stride(&mem, 2, 2, 6)
    ///     .expect("Memory region too small");
    /// ```
    pub fn with_stride(raw: &'a [u8], width: u32, height: u32, stride: usize) -> Option<Self> {
        let len = height as usize * stride;

        if stride > 0 && raw.len() != len {
            return None;
        }

        Some(DynamicView {
            raw: MemoryView::U8(raw),
            width,
            height,
            stride,
            typ: StorageType::U8,
        })
    }
}

/// Image buffer parametrized by its pixel type
pub struct DynamicBuffer {
    pub raw: MemoryBuffer,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub typ: StorageType,
}

impl DynamicBuffer {
    /// Returns an image view with unknown pixel type
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
    /// use ffimage::packed::{DynamicImageBuffer, DynamicStorageType};
    ///
    /// let buf = DynamicImageBuffer::new(2, 2, 3, DynamicStorageType::U8);
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

        DynamicBuffer {
            raw,
            width,
            height,
            stride,
            typ,
        }
    }

    /// Returns an image view with unknown pixel type
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `channels` - Number of channels
    /// * `typ` - Storage type
    /// * `raw` - Raw memory region to interpret as typed image
    ///
    /// # Example
    ///
    /// ```
    /// use ffimage::packed::DynamicImageBuffer;
    ///
    /// let mem = vec![0; 12];
    /// let buf = DynamicImageBuffer::with_raw(2, 2, 3, &mem);
    /// ```
    pub fn with_raw(width: u32, height: u32, channels: u32, raw: &[u8]) -> Self {
        let stride = width as usize * channels as usize;
        DynamicBuffer {
            raw: MemoryBuffer::U8(raw.to_vec()),
            width,
            height,
            stride,
            typ: StorageType::U8,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, channels: u32) {
        self.width = width;
        self.height = height;
        self.stride = width as usize * channels as usize * self.typ as usize;

        let elems = (height * width * channels) as usize * self.typ as usize;
        match &mut self.raw {
            MemoryBuffer::U8(buf) => buf.resize(elems, 0),
            MemoryBuffer::U16(buf) => buf.resize(elems, 0),
        }
    }
}

impl<'a, T> TryFrom<&DynamicView<'a>> for GenericView<'a, T>
where
    T: Pixel<T = u8>,
{
    type Error = ();

    fn try_from(input: &DynamicView<'a>) -> Result<Self, Self::Error> {
        let mem: &'a [u8];
        match input.raw {
            MemoryView::U8(view) => mem = view,
            _ => return Err(()),
        }

        let view = GenericView::<T>::new(mem, input.width, input.height);
        match view {
            Some(view) => Ok(view),
            None => Err(()),
        }
    }
}
