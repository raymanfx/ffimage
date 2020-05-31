use std::cell::UnsafeCell;
use std::convert::From;

use rayon::prelude::*;

use crate::core::traits::{Convert, ImageBuffer, ImageView, Pixel, Resize, TryConvert};
use crate::packed::image::{GenericBuffer, GenericFlatBuffer, GenericView};

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
            DP: Pixel + From<SP>,
        {
            fn convert(&self, output: &mut $dst<'a, DP>) {
                output.resize(self.width(), self.height());

                // It is safe to use the shared, lock free wrapper here because each thread
                // accesses a distinct pixel row, so pixel access is never interleaved.
                let output = UnsafeShared::new(output);

                (0..self.height()).into_par_iter().for_each(|i| {
                    for j in 0..self.width() {
                        let output = output.get();
                        let src_pix = self.get_pixel(j, i).unwrap();
                        let dst_pix = DP::from(src_pix);
                        output.set_pixel(j, i, &dst_pix).unwrap();
                    }
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
            DP: Pixel + From<SP>,
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
                    for j in 0..self.width() {
                        let output = output.get();
                        let src_pix = self.get_pixel(j, i).unwrap();
                        let dst_pix = DP::from(src_pix);
                        output.set_pixel(j, i, &dst_pix).unwrap();
                    }
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
