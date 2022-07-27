use std::mem;

use crate::packed::Image;
use crate::traits::{GenericImageView, Pixel};

#[derive(Debug)]
/// An iterator type for images to iterate through pixels
pub struct PixelIter<T> {
    pub img: T,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl<'a, T, B> Iterator for PixelIter<&'a Image<T, B>>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        let x = self.x;
        self.x += 1;

        Some(&self.img[self.y as usize][x as usize])
    }
}

impl<'a, T, B> IntoIterator for &'a Image<T, B>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]>,
{
    type Item = &'a T;
    type IntoIter = PixelIter<&'a Image<T, B>>;

    fn into_iter(self) -> Self::IntoIter {
        let width = self.width();
        let height = self.height();

        PixelIter {
            img: self,
            x: 0,
            y: 0,
            width,
            height,
        }
    }
}

impl<'a, T, B> Iterator for PixelIter<&'a mut Image<T, B>>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        let x = self.x;
        self.x += 1;

        // This is safe because...
        // (from http://stackoverflow.com/questions/25730586):
        // The Rust compiler does not know that when you ask a mutable iterator for the next
        // element, that you get a different reference every time and never the same reference
        // twice. Of course, we know that such an iterator won't give you the same reference twice.
        unsafe { mem::transmute(&self.img[self.y as usize][x as usize]) }
    }
}

impl<'a, T, B> IntoIterator for &'a mut Image<T, B>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    type Item = &'a mut T;
    type IntoIter = PixelIter<&'a mut Image<T, B>>;

    fn into_iter(self) -> Self::IntoIter {
        let width = self.width();
        let height = self.height();

        PixelIter {
            img: self,
            x: 0,
            y: 0,
            width,
            height,
        }
    }
}

#[derive(Debug)]
/// An iterator type for images to iterate through pixel rows
pub struct RowIter<T> {
    pub img: T,
    pub y: u32,
    pub height: u32,
}

impl<'a, T, B> Iterator for RowIter<&'a Image<T, B>>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]>,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.y as usize;

        self.y += 1;
        if self.y > self.height {
            return None;
        }

        Some(&self.img[row])
    }
}

impl<'a, T, B> Iterator for RowIter<&'a mut Image<T, B>>
where
    T: Pixel + Copy,
    B: AsRef<[T::T]> + AsMut<[T::T]>,
{
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.y as usize;

        self.y += 1;
        if self.y > self.height {
            return None;
        }

        // This is safe because...
        // (from http://stackoverflow.com/questions/25730586):
        // The Rust compiler does not know that when you ask a mutable iterator for the next
        // element, that you get a different reference every time and never the same reference
        // twice. Of course, we know that such an iterator won't give you the same reference twice.
        unsafe { Some(mem::transmute(&mut self.img[row])) }
    }
}
