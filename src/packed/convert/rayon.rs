use std::cell::UnsafeCell;

use rayon::prelude::*;

use crate::core::traits::{Convert, ImageView, Pixel, Resize, TryConvert, TryConvertSlice};
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

macro_rules! impl_Convert {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> Convert<$dst<'a, DP>> for $src<'a, SP>
        where
            SP: Pixel,
            DP: Pixel,
            [SP]: TryConvertSlice<DP>,
            <[SP] as TryConvertSlice<DP>>::Error: std::fmt::Debug,
        {
            fn convert(&self, output: &mut $dst<'a, DP>) {
                output.resize(self.width(), self.height());

                // It is safe to use the shared, lock free wrapper here because each thread
                // accesses a distinct pixel row, so pixel access is never interleaved.
                let output = UnsafeShared::new(output);

                (0..self.height()).into_par_iter().for_each(|i| {
                    let output = output.get();
                    let row_in = self.pixel_row(i).unwrap();
                    let row_out = output.pixel_row_mut(i).unwrap();
                    row_in.try_convert(row_out).unwrap();
                });
            }
        }
    };
}

macro_rules! impl_TryConvert {
    ($src:ident, $dst:ident) => {
        impl<'a, SP, DP> TryConvert<$dst<'a, DP>> for $src<'a, SP>
        where
            SP: Pixel,
            DP: Pixel,
            [SP]: TryConvertSlice<DP>,
            <[SP] as TryConvertSlice<DP>>::Error: std::fmt::Debug,
        {
            type Error = ();

            fn try_convert(&self, output: &mut $dst<'a, DP>) -> Result<(), Self::Error> {
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
                    row_in.try_convert(row_out).unwrap();
                });

                Ok(())
            }
        }
    };
}

impl_Convert!(GenericView, GenericBuffer);
impl_Convert!(GenericFlatBuffer, GenericBuffer);
impl_Convert!(GenericBuffer, GenericBuffer);

impl_TryConvert!(GenericView, GenericFlatBuffer);
impl_TryConvert!(GenericFlatBuffer, GenericFlatBuffer);
impl_TryConvert!(GenericBuffer, GenericFlatBuffer);
