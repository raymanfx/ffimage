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
//! use ffimage::packed::{ImageView, ImageBuffer};
//! use ffimage::color::{Rgb, Gray};
//! use ffimage::traits::Convert;
//!
//! // This is our grayscale image memory.
//! // Usually, this will be allocated by a foreign function (e.g. kernel driver) and contain
//! // read-only memory.
//! let mem: [u8; 12] = [0; 12];
//!
//! // Create a statically typed view of the image, assuming it is RGB 24 bits per pixel.
//! // The u8 parameter denotes the internal storage type used by image pixels. In our case, each
//! // channel requires eight bits, which makes for a total of 3 * 8 = 24 bits per pixel.
//! // The length of the memory slice is validated and a None value is returned when constraints
//! // are violated.
//! let view = ImageView::<Rgb<u8>>::from_buf(&mem, 2, 2).unwrap();
//!
//! // Create a target buffer for the destination image.
//! // Here we initialize an empty buffer with width and height both being zero. This is fine since
//! // the `Convert` trait implementation will resize the target buffer for us.
//! let mut buf = ImageBuffer::<Gray<u8>>::new(0, 0, 0u8);
//!
//! // Perform the actual conversion.
//! // This cannot fail since the target buffer is resizable.
//! // If the pixel conversion between source and target image is not defined, the compiler will
//! // refuse to compile this line.
//! view.convert(&mut buf);
//!```

pub mod pixel;
pub mod traits;

pub mod color;
pub mod packed;
