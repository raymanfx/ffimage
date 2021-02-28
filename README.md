# Foreign function image handling

[![license](https://img.shields.io/github/license/raymanfx/ffimage?style=for-the-badge)](https://github.com/raymanfx/ffimage/blob/master/LICENSE.txt)
[![Build Status](https://img.shields.io/travis/raymanfx/ffimage/master.svg?style=for-the-badge&logo=travis)](https://travis-ci.org/raymanfx/ffimage)

This crate provides easy image handling and conversion capabilities in Rust.
It is designed to work with image buffers originating from foreign functions, which could be a C API or a camera driver which maps device buffers into userspace.

There are three main struct types in this crate: views, flat buffers and buffers.
A view is a read-only image representation, whereas a buffer is writable since it owns its backing memory. Finally, flat buffers are writable as well but take (mutable) existing memory regions to operate on.

## Goals
This crate shall facilitate zero-allocation image buffer conversion through views flat buffers. Additionally, full buffers can be created when a new output buffer is needed or the existing buffer is to small.

There are two main APIs: `packed` for traditional packed images and `planar` for memory-to-memory (M2M) usecases. See [Packed and Planar APIs](#packed-and-planar-apis) for more information.

## Packed and Planar APIs
Two distinct APIs are planned: packed and planar. Packed images have their pixels reside beneath each other in memory while planar images require separate memory planes for pixel components.
For example, a packed RGB image would look like this in memory:

..|RGB|RGB|RGB|.. (single memory plane)

whereas a planar RGB image would look like this:

..|RRR|GGG|BBB|.. (three memory planes)

## Usage
Below you can find a quick example usage of this crate. It introduces the basics necessary for image conversion.

```rust
use ffimage::packed::{ImageView, ImageBuffer};
use ffimage::color::{Rgb, Gray};
use ffimage::core::Convert;

fn main() {
    // This is our grayscale image memory.
    // Usually, this will be allocated by a foreign function (e.g. kernel driver) and contain
    // read-only memory.
    let mem: [u8; 12] = [0; 12];

    // Create a statically typed view of the image, assuming it is RGB 24 bits per pixel.
    // The u8 parameter denotes the internal storage type used by image pixels. In our case, each
    // channel requires eight bits, which makes for a total of 3 * 8 = 24 bits per pixel.
    // The length of the memory slice is validated and a None value is returned when constraints
    // are violated.
    let view = ImageView::<Rgb<u8>>::from_buf(&mem, 2, 2).unwrap();

    // Create a target buffer for the destination image.
    // Here we initialize an empty buffer with width and height both being zero. This is fine since
    // the `Convert` trait implementation will resize the target buffer for us.
    let mut buf = ImageBuffer::<Gray<u8>>::new(0, 0, 0u8);

    // Perform the actual conversion.
    // This cannot fail since the target buffer is resizable.
    // If the pixel conversion between source and target image is not defined, the compiler will
    // refuse to compile this line.
    view.convert(&mut buf);
}
```

Have a look at the provided `examples` for more sample applications.
