#[macro_export]
/// Implement the Pixel trait for a pixel
macro_rules! impl_Pixel {
    ($name:ident, $channels:expr) => {
        impl<T: StorageType> Pixel for $name<T> {
            type T = T;

            fn at(&self, index: usize) -> Self::T {
                self.0[index]
            }

            fn try_from(raw: &[Self::T]) -> Result<Self, array::TryFromSliceError> {
                match <[T; $channels]>::try_from(raw) {
                    Ok(components) => Ok($name { 0: components }),
                    Err(e) => Err(e),
                }
            }

            fn channels() -> u8 {
                $channels
            }
        }

        impl<T: StorageType> std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, i: usize) -> &Self::Output {
                &self.0[i]
            }
        }

        impl<T: StorageType> std::ops::IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                &mut self.0[i]
            }
        }
    };
}

#[macro_export]
/// Define a new pixel struct
macro_rules! define_pixel {
    ($name:ident, $channels:expr, #[$doc:meta]) => {
        #[repr(C)]
        #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
        #[$doc]
        pub struct $name<T: StorageType>(pub [T; $channels]);

        impl<T: StorageType> $name<T> {
            /// Returns a new pixel
            ///
            /// # Arguments
            ///
            /// * `channels` - Channel values
            ///
            /// # Example
            ///
            /// ```
            /// use std::array;
            /// use std::convert::{From, TryFrom};
            /// use std::ops::{Index, IndexMut};
            ///
            /// use ffimage::{create_pixel, define_pixel, impl_Pixel};
            /// use ffimage::core::{Pixel, StorageType};
            ///
            /// // define a new pixel type
            /// create_pixel!(Rgb, 3, #[doc = "RGB pixel"]);
            ///
            /// // use the newly created type
            /// let pix = Rgb::<u8>::new([0, 0, 0]);
            /// ```
            pub fn new(channels: [T; $channels]) -> Self {
                $name { 0: channels }
            }
        }
    };
}

#[macro_export]
/// Create a new pixel type
///
/// A pixel is defined by its number of channels and the storage type (which is the same for each
/// channel). The memory layout is C compatible by default and the Debug, Copy and Clone traits
/// are derived. The Pixel trait is implemented automatically providing the basic building blocks
/// for higher level structures such as image view and buffer types.
///
/// # Example
///
/// ```
/// use std::array;
/// use std::convert::{From, TryFrom};
/// use std::ops::{Index, IndexMut};
///
/// use ffimage::{create_pixel, define_pixel, impl_Pixel};
/// use ffimage::core::traits::{Pixel, StorageType};
///
/// create_pixel!(Rgb, 3, #[doc = "RGB pixel"]);
/// ```
macro_rules! create_pixel {
    ($name:ident, $channels:expr, #[$doc:meta]) => {
        define_pixel!($name, $channels, #[$doc]);
        impl_Pixel!($name, $channels);
    };
}
