use std::cell::UnsafeCell;

use rayon::prelude::*;

use crate::convert::{Convert, MapPixels};
use crate::packed::Image;
use crate::traits::{GenericImageView, Pixel};

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

fn _convert<SP, DP, T, U>(input: &Image<SP, T>, output: &mut Image<DP, U>)
where
    SP: Pixel + Copy + MapPixels<SP, DP> + Sync,
    DP: Pixel + Copy + Send,
    SP::T: Sync,
    DP::T: Send + Sync,
    T: AsRef<[SP::T]> + Sync,
    U: AsRef<[DP::T]> + AsMut<[DP::T]> + Send,
{
    let rows = if input.height() < output.height() {
        input.height() as usize
    } else {
        output.height() as usize
    };

    // It is safe to use the shared, lock free wrapper here because each thread
    // accesses a distinct pixel row, so pixel access is never interleaved.
    let output = UnsafeShared::new(output);

    (0..rows).into_par_iter().for_each(|i| {
        let output = unsafe { output.get() };

        SP::map_pixels(input[i].as_ref(), output[i].as_mut())
    })
}

impl<SP, DP, T, U> Convert<Image<DP, U>> for Image<SP, T>
where
    SP: Pixel + Copy + MapPixels<SP, DP> + Sync,
    DP: Pixel + Copy + Send,
    SP::T: Sync,
    DP::T: Send + Sync,
    T: AsRef<[SP::T]> + Sync,
    U: AsRef<[DP::T]> + AsMut<[DP::T]> + Send,
{
    fn convert(&self, output: &mut Image<DP, U>) {
        _convert(self, output)
    }
}
