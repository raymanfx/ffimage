/// Generic pixel container
pub trait Pixel {
    /// Type of the container elements
    type T;

    /// Number of channels for this pixel
    fn channels() -> u8;

    /// Number of image pixels for this pixel
    fn subpixels() -> u8;
}
