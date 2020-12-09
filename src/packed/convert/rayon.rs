use std::{cell::UnsafeCell, ops::Index};

use rayon::prelude::*;

use crate::core::traits::{GenericImageView, Pixel, Convert};
use crate::packed::traits::ConvertPixels;
use crate::packed::generic::{ImageViewMut, ImageBuffer};

// This is a private helper struct to share buffers between threads in a lock free manner where we
// would usually need a Mutex. Only use this when you can ensure that all usage of the wrapped
// value is safe and never interleaved!
struct UnsafeShared<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T> UnsafeShared<T> {
    pub fn new(t: T) -> UnsafeShared<T> {
        UnsafeShared {
            value: UnsafeCell::new(t),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get(&self) -> &mut T {
        &mut *self.value.get()
    }
}

unsafe impl<T: ?Sized + Send> Send for UnsafeShared<T> {}
unsafe impl<T: ?Sized + Send> Sync for UnsafeShared<T> {}

impl<'a, 'b, DP, I> Convert<ImageViewMut<'b, DP>> for I
where
DP: Pixel,
I: GenericImageView<'a> + Index<usize> + Sync,
for <'c> &'c mut <ImageBuffer<DP> as Index<usize>>::Output: IntoIterator,
for <'c> &'c <Self as Index<usize>>::Output: ConvertPixels<&'c mut <ImageBuffer<DP> as Index<usize>>::Output>,
{
    fn convert(&self, output: &mut ImageViewMut<'b, DP>) {
        // how many rows can we convert?
        let row_count = if output.height() > self.height() {
            self.height()
        } else {
            output.height()
        };

        // It is safe to use the shared, lock free wrapper here because each thread
        // accesses a distinct pixel row, so pixel access is never interleaved.
        let output = UnsafeShared::new(output);

        (0..row_count as usize).into_par_iter().for_each(|i| {
            let output = unsafe { output.get() };
            let input = &self[i];
            let output = &mut output[i];
            input.convert(output);
        });
    }
}

impl<'a, DP, I> Convert<ImageBuffer<DP>> for I
where
DP: Pixel,
I: GenericImageView<'a> + Index<usize> + Sync,
for <'b> &'b mut <ImageBuffer<DP> as Index<usize>>::Output: IntoIterator,
for <'b> &'b <Self as Index<usize>>::Output: ConvertPixels<&'b mut <ImageBuffer<DP> as Index<usize>>::Output>,
{
    fn convert(&self, output: &mut ImageBuffer<DP>) {
        if output.width() != self.width() || output.height() != self.height() {
            *output = ImageBuffer::new(self.width(), self.height());
        }

        // It is safe to use the shared, lock free wrapper here because each thread
        // accesses a distinct pixel row, so pixel access is never interleaved.
        let output = UnsafeShared::new(output);

        (0..self.height() as usize).into_par_iter().for_each(|i| {
            let output = unsafe { output.get() };
            let input = &self[i];
            let output = &mut output[i];
            input.convert(output);
        });
    }
}
