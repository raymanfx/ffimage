use std::cell::UnsafeCell;

use rayon::prelude::*;

use crate::core::traits::{GenericImageView, Pixel, Convert, ConvertSlice};
use crate::packed::generic::{ImageView, ImageViewMut, ImageBuffer};

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

macro_rules! impl_Convert {
    () => {
        fn convert(&self, output: &mut ImageBuffer<DP>) {
            if output.width() != self.width() || output.height() != self.height() {
                *output = ImageBuffer::new(self.width(), self.height());
            }

            let row_count = output.height();

            // It is safe to use the shared, lock free wrapper here because each thread
            // accesses a distinct pixel row, so pixel access is never interleaved.
            let output = UnsafeShared::new(output);

            (0..row_count).into_par_iter().for_each(|i| {
                let output = unsafe { output.get() };
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                SP::convert(row_in, row_out);
            });
        }
    };
}

macro_rules! impl_ConvertFlat {
    () => {
        fn convert(&self, output: &mut ImageViewMut<'b, DP>) {
            let row_count = if output.height() < self.height() {
                output.height()
            } else {
                self.height()
            };

            // It is safe to use the shared, lock free wrapper here because each thread
            // accesses a distinct pixel row, so pixel access is never interleaved.
            let output = UnsafeShared::new(output);

            (0..row_count).into_par_iter().for_each(|i| {
                let output = unsafe { output.get() };
                let row_in = &self[i as usize];
                let row_out = &mut output[i as usize];
                SP::convert(row_in, row_out);
            });
        }
    };
}

impl<'a, SP, DP> Convert<ImageBuffer<DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, SP, DP> Convert<ImageBuffer<DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<SP, DP> Convert<ImageBuffer<DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_Convert!();
}

impl<'a, 'b, SP, DP> Convert<ImageViewMut<'b, DP>> for ImageView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}

impl<'a, 'b, SP, DP> Convert<ImageViewMut<'b, DP>> for ImageViewMut<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}

impl<'b, SP, DP> Convert<ImageViewMut<'b, DP>> for ImageBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    SP: ConvertSlice<DP>,
{
    impl_ConvertFlat!();
}
