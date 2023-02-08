//! ffimage is a crate for foreign-function image handling and conversion.
//!
//! It features basic pixel abstractions and allows converting between color formats. New pixel
//! types can easily be defined and used with the existing abstractions. By default, RGB / BGR,
//! Grayscale as well as YUV 4:4:4, 4:2:2, 4:2:0p (planar) are supported.
//!
//! Additional documentation can currently also be found in the
//! [README.md file which is most easily viewed on github](https://github.com/raymanfx/ffimage/blob/master/README.md).
//!
//! [Jump forward to crate content](#reexports)
//!
//! # Overview
//!
//! The common user of this crate will mainly be interested in image conversion.
//! This is a very brief example of RGB -> Grayscale conversion of existing memory:
//!
//! ```no_run
//! use ffimage::color::{Rgb, Gray};
//!
//! // This is our RGB image memory.
//! // Usually, this will be allocated by a foreign function (e.g. kernel driver) and contain
//! // read-only memory.
//! let rgb = vec![Rgb::<u8>([10, 10, 10]); 10];
//!
//! // Convert the pixels into Grayscale pixels by mapping each one individually.
//! let gray: Vec<Gray<u8>> = rgb
//!     .iter()
//!     .copied()
//!     .map(|rgb| Gray::<u8>::from(rgb))
//!     .collect();
//!```

#![no_std]

/// Generic pixel attributes
pub trait Pixel {
    /// Number of channels for this pixel
    const CHANNELS: u8;
    /// Number of image pixels for this pixel
    const SUBPIXELS: u8 = 1;
}

pub mod color;
pub mod iter;
