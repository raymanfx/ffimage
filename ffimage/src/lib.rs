//! ffimage is a crate for foreign-function image handling and conversion.
//!
//! It features basic image abstractions and allows converting between views (non-owning) and
//! buffers (owning) representations. The core building block is the Pixel trait. New pixel types
//! can easily be defined by using procedural macros.
//!
//! Additional documentation can currently also be found in the
//! [README.md file which is most easily viewed on github](https://github.com/raymanfx/ffimage/blob/master/README.md).
//!
//! [Jump forward to crate content](#reexports)
//!
//! # Overview
//!
//! Images are laid out in a certain way in memory. Most of the time, they are packed, meaning one
//! pixel closely follows the other, with each channel in sequence. For example, RGB pixels would
//! usually be laid out like this in a packed image:
//!
//! R|G|B|R|G|B|..
//!
//! There are other formats with other memory layouts though, in particular video frames. They
//! usually feature something called memory 'planes'. Each plane is a linear vector in memory.
//! This is especially common for YCbCr/YUV like formats.
//!
//! The common user of this crate will mainly be interested in image conversion.
//! Here is a very brief example of RGB -> Grayscale conversion of existing memory:
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
    fn channels() -> u8;

    /// Number of image pixels for this pixel
    fn subpixels() -> u8;
}

pub mod color;
pub mod iter;
