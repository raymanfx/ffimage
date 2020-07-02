use std::cell::UnsafeCell;

use rayon::prelude::*;

use crate::core::traits::{ImageView, Pixel, Resize, TryConvert, TryConvertSlice};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};
use crate::packed::traits::{AccessPixel, AccessPixelMut};

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
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

unsafe impl<T: ?Sized + Send> Send for UnsafeShared<T> {}
unsafe impl<T: ?Sized + Send> Sync for UnsafeShared<T> {}

macro_rules! impl_TryConvert {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericBuffer<DP>) -> Result<(), Self::Error> {
            output.resize(self.width(), self.height());

            // It is safe to use the shared, lock free wrapper here because each thread
            // accesses a distinct pixel row, so pixel access is never interleaved.
            let output = UnsafeShared::new(output);

            (0..self.height()).into_par_iter().for_each(|i| {
                let output = output.get();
                let row_in = self.pixel_row(i).unwrap();
                let row_out = output.pixel_row_mut(i).unwrap();
                // TODO: marshal error
                row_in.try_convert(row_out).unwrap();
            });

            Ok(())
        }
    };
}

macro_rules! impl_TryConvertFlat {
    () => {
        type Error = ();

        fn try_convert(&self, output: &mut GenericFlatBuffer<'b, DP>) -> Result<(), Self::Error> {
            if output.width() < self.width() || output.height() < self.height() {
                return Err(());
            }

            // It is safe to use the shared, lock free wrapper here because each thread
            // accesses a distinct pixel row, so pixel access is never interleaved.
            let output = UnsafeShared::new(output);

            (0..self.height()).into_par_iter().for_each(|i| {
                let output = output.get();
                let row_in = self.pixel_row(i).unwrap();
                let row_out = output.pixel_row_mut(i).unwrap();
                // TODO: marshal error
                row_in.try_convert(row_out).unwrap();
            });

            Ok(())
        }
    };
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, SP, DP> TryConvert<GenericBuffer<DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<SP, DP> TryConvert<GenericBuffer<DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvert!();
}

impl<'a, 'b, SP, DP> TryConvert<GenericFlatBuffer<'b, DP>> for GenericView<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'a, 'b, SP, DP> TryConvert<GenericFlatBuffer<'b, DP>> for GenericFlatBuffer<'a, SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}

impl<'b, SP, DP> TryConvert<GenericFlatBuffer<'b, DP>> for GenericBuffer<SP>
where
    SP: Pixel,
    DP: Pixel,
    [SP]: TryConvertSlice<DP>,
{
    impl_TryConvertFlat!();
}
