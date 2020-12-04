extern crate ffimage;

use ffimage::prelude::*;

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
    let view = PackedImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();

    // Create a target buffer for the destination image.
    // Here we initialize an empty buffer with width and height both being zero. This is fine since
    // the `Convert` trait implementation will resize the target buffer for us.
    let mut buf = PackedImageBuffer::<Gray<u8>>::new(0, 0);

    // Perform the actual conversion.
    // If the pixel conversion between source and target image is not defined, the compiler will
    // refuse to compile this line.
    view.convert(&mut buf);
}
