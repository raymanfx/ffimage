extern crate ffimage;

use std::convert::TryFrom;

use ffimage::prelude::*;

fn main() {
    // This is our grayscale image memory.
    // Usually, this will be allocated by a foreign function (e.g. kernel driver) and contain
    // read-only memory.
    let mem: [u8; 12] = [0; 12];

    // A dynamic view represents an image buffer just like a generic view does, but the format does
    // not have to be known at compile time. Instead, these views can be converted into generic
    // views at runtime.
    let dynamic_view = DynamicImageView::<u8>::new(&mem, 2, 2, "Rgb", 3).unwrap();

    // Create a statically typed view of the image, assuming it is RGB 24 bits per pixel.
    // The u8 parameter denotes the internal storage type used by image pixels. In our case, each
    // channel requires eight bits, which makes for a total of 3 * 8 = 24 bits per pixel.
    // The length of the memory slice is validated and a None value is returned when constraints
    // are violated.
    let generic_view = GenericImageView::<Rgb<u8>>::try_from(&dynamic_view).unwrap();

    // Create a target buffer for the destination image.
    // Here we initialize an empty buffer with width and height both being zero. This is fine since
    // the `Convert` trait implementation will resize the target buffer for us.
    let mut buf = GenericImageBuffer::<Gray<u8>>::new(0, 0);

    // Perform the actual conversion.
    // This cannot fail since the target buffer is resizable.
    // If the pixel conversion between source and target image is not defined, the compiler will
    // refuse to compile this line.
    generic_view.convert(&mut buf);
}
