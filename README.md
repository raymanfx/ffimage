# Foreign function image handling

[![license](https://img.shields.io/github/license/raymanfx/ffimage?style=for-the-badge)](https://github.com/raymanfx/ffimage/blob/master/LICENSE.txt)
[![Build Status](https://img.shields.io/github/actions/workflow/status/raymanfx/ffimage/ci.yml?branch=master&logo=github&style=for-the-badge)](https://github.com/raymanfx/ffimage/actions)

This crate provides easy image pixel handling and conversion capabilities in Rust.
It is designed to work with image buffers originating from foreign functions, which could be a C API or a camera driver which maps device buffers into userspace.

Pixels are represented as array wrapper types, e.g. `[T; 3]` would be the inner type for an `Rgb` pixel. The crate provides iterator extensions so you can easily convert between color formats, e.g. `Yuv` and `Rgb`.

## Packed and Planar APIs
Packed images have their pixels reside beneath each other in memory while planar images require separate memory planes for pixel components.

For example, a packed RGB image would look like this in memory:

R<sub>1</sub>G<sub>1</sub>B<sub>1</sub> .. R<sub>n</sub>G<sub>n</sub>n<sub>n</sub> (single memory plane)

whereas a planar YUV420p image would look like this:

Y<sub>1</sub>Y<sub>2</sub> .. Y<sub>n</sub> | U<sub>1</sub>U<sub>2</sub> .. U<sub>n</sub> | V<sub>1</sub>V<sub>2</sub> .. V<sub>n</sub> (three memory planes)

## Usage
Below you can find a quick example usage of this crate. It introduces the basics necessary for image conversion.

```rust
use ffimage::color::{Gray, Rgb};
use ffimage::iter::{ColorConvertExt, PixelsExt, WriteExt};

fn main() {
    // This is our RGB image memory (2x2 pixels).
    // Usually, this will be allocated by a foreign function (e.g. kernel driver) and contain
    // read-only memory.
    let rgb = [10; 4 * 3];
    // We need an output buffer as well to host the converted grayscale pixels.
    let mut gray = [0; 4 * 1];

    // Convert from rgb to grayscale by mapping each pixel. The Pixels iterator extension creates
    // a typed pixel iterator from a bytestream. The ColorConvert extension knows how to convert
    // between pixel types and the Write extension finally writes the pixels back into a
    // bytestream.
    rgb.iter()
        .copied()
        .pixels::<Rgb<u8>>()
        .colorconvert::<Gray<u8>>()
        .write(&mut gray);
}
```

## Benchmark
A benchmark suite is included in this crate. Run it using
```
$ cargo bench
```

These are my results for `ffimage v0.10.0` on a MacBook Pro 14" (M1 Pro):
| In         | Out      | 640x480   | 1280x720  |
|------------|----------|-----------|-----------|
| Rgb[u8]    | Bgr[u8]  | 18.028 µs | 53.882 µs |
| Rgb[u8]    | Gray[u8] | 381.48 µs | 1.1442 ms |
| Yuv[u8]    | Rgb[u8]  | 165.32 µs | 496.33 µs |
| Yuv422[u8] | Rgb[u8]  | 1.2097 ms | 3.6284 ms |
