#[macro_export]
/// Implement the Pixel trait for a pixel
macro_rules! impl_Pixel {
    ($name:ident, $channels:expr) => {
        impl<T: Copy + Send> Pixel for $name<T> {
            type T = T;

            fn channels() -> u8 {
                $channels
            }
        }

        impl<T> std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, i: usize) -> &Self::Output {
                &self.0[i]
            }
        }

        impl<T> std::ops::IndexMut<usize> for $name<T> {
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
        pub struct $name<T>(pub [T; $channels]);

        impl<T> $name<T> {
            /// Returns a new pixel
            ///
            /// # Arguments
            ///
            /// * `channels` - Channel values
            ///
            /// # Example
            ///
            /// ```
            /// use ffimage::{create_pixel, define_pixel, impl_Pixel};
            /// use ffimage::core::Pixel;
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

        impl<T> From<[T; $channels]> for $name<T>
        where
            T: Copy,
        {
            fn from(array: [T; $channels]) -> Self {
                $name { 0: array }
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
/// use ffimage::{create_pixel, define_pixel, impl_Pixel};
/// use ffimage::core::traits::Pixel;
///
/// create_pixel!(Rgb, 3, #[doc = "RGB pixel"]);
/// ```
macro_rules! create_pixel {
    ($name:ident, $channels:expr, #[$doc:meta]) => {
        define_pixel!($name, $channels, #[$doc]);
        impl_Pixel!($name, $channels);
    };
}
