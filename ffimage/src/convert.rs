/// Map pixels from one format to another in place
pub trait MapPixels<T, U> {
    /// Converts a number of pixels into a (possibly different) number of pixels with another format
    fn map_pixels<'a, I, O: IntoIterator>(input: I, output: O)
    where
        I: IntoIterator<Item = &'a T>,
        O: IntoIterator<Item = &'a mut U>,
        T: 'a,
        U: 'a;
}

// Blanket implementation for pixel conversion.
// If we know how to convert a single pixel into another one, we can automatically convert between
// iterators of them as well. This obviously does not work for macropixels, where one pixel may
// transform into several, so you need to implement the trait yourself for those types.
impl<T, U> MapPixels<T, U> for T
where
    T: Copy,
    U: From<T>,
{
    fn map_pixels<'a, I, O>(input: I, output: O)
    where
        I: IntoIterator<Item = &'a T>,
        O: IntoIterator<Item = &'a mut U>,
        T: 'a,
        U: 'a,
    {
        input
            .into_iter()
            .zip(output.into_iter())
            .for_each(|(t, u)| *u = U::from(*t))
    }
}

/// Convert between images
pub trait Convert<B> {
    /// Converts the buffer into another, possibly with a different format
    fn convert(&self, output: &mut B);
}
