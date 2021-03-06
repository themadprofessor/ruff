//! Pixels within a Farbfeld image as defined by the [spec](http://tools.suckless.org/farbfeld/).

use std::iter::{ExactSizeIterator, FusedIterator};

/// A single pixel in a Farbfeld image as defined by the [spec](http://tools.suckless.org/farbfeld/)
/// by Suckless.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pixel {
    red: u16,
    green: u16,
    blue: u16,
    alpha: u16
}

/// A consuming iterator implementation for [Pixel](struct.Pixel.html). It produces the red, green,
/// blue then alpha values of a pixel in that order, then returns None.
///
/// # Examples
/// ```
/// # use ::ruff::*;
/// let mut iter = Pixel::new(10_u16, 20_u16, 30_u16, 40_u16).into_iter();
///
/// assert_eq!(Some(10_u16), iter.next());
/// assert_eq!(Some(20_u16), iter.next());
/// assert_eq!(Some(30_u16), iter.next());
/// assert_eq!(Some(40_u16), iter.next());
/// assert_eq!(None, iter.next());
/// ```
#[derive(Debug, Clone)]
pub struct IntoIter {
    pixel: Pixel,
    curr: u8
}

/// A non-consuming iterator implementation for [Pixel](struct.Pixel.html). It produces a reference
/// to the red, green, blue then alpha values of a pixel, then returns None.
///
/// # Examples
/// ```
/// # use ::ruff::*;
/// let pixel = Pixel::new(10_u16, 20_u16, 30_u16, 40_u16);
/// let mut iter = pixel.iter();
///
/// assert_eq!(Some(&10_u16), iter.next());
/// assert_eq!(Some(&20_u16), iter.next());
/// assert_eq!(Some(&30_u16), iter.next());
/// assert_eq!(Some(&40_u16), iter.next());
/// assert_eq!(None, iter.next());
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    pixel: &'a Pixel,
    curr: u8
}

impl Pixel {
    /// Creates a new Pixel.
    pub fn new<T>(red: T, green: T, blue: T, alpha: T) -> Pixel where T: Into<u16> {
        Pixel {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
            alpha: alpha.into()
        }
    }

    /// Returns a reference to the red component of this pixel.
    pub fn red(&self) -> &u16 {
        &self.red
    }

    /// Returns a reference to the green component of this pixel.
    pub fn green(&self) -> &u16 {
        &self.green
    }

    /// Returns a reference to the blue component of this pixel.
    pub fn blue(&self) -> &u16 {
        &self.blue
    }

    /// Returns a reference to the alpha component of this pixel.
    pub fn alpha(&self) -> &u16 {
        &self.alpha
    }

    /// Returns a mutable reference to the red component of this pixel.
    pub fn red_mut(&mut self) -> &mut u16 {
        &mut self.red
    }

    /// Returns a mutable reference to the greeen component of this pixel.
    pub fn green_mut(&mut self) -> &mut u16 {
        &mut self.green
    }

    /// Returns a mutable reference to the blue component of this pixel.
    pub fn blue_mut(&mut self) -> &mut u16 {
        &mut self.blue
    }

    /// Returns a mutable reference to the alpha component of this pixel.
    pub fn alpha_mut(&mut self) -> &mut u16 {
        &mut self.alpha
    }

    /// Creates an iterator over a reference to the slice. The iterator produces a reference to the
    /// red, green, blue then alpha component of this pixel, then returns None.
    pub fn iter(&self) -> Iter {
        Iter {pixel: self, curr: 0}
    }
}

impl From<[u16; 4]> for Pixel {
    fn from(i: [u16; 4]) -> Self {
        Pixel {
            red: i[0],
            green: i[1],
            blue: i[2],
            alpha: i[3]
        }
    }
}

impl Into<[u16; 4]> for Pixel {
    fn into(self) -> [u16; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl Iterator for IntoIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            0 => {self.curr += 1; Some(self.pixel.red)},
            1 => {self.curr += 1; Some(self.pixel.green)},
            2 => {self.curr += 1; Some(self.pixel.blue)},
            3 => {self.curr += 1; Some(self.pixel.alpha)},
            _ => None
        }
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        4 - self.curr as usize
    }
}

impl FusedIterator for IntoIter {}

impl <'a> Iterator for Iter<'a> {
    type Item = &'a u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            0 => {self.curr += 1; Some(&self.pixel.red)},
            1 => {self.curr += 1; Some(&self.pixel.green)},
            2 => {self.curr += 1; Some(&self.pixel.blue)},
            3 => {self.curr += 1; Some(&self.pixel.alpha)},
            _ => None
        }
    }
}

impl <'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        4 - self.curr as usize
    }
}

impl <'a> FusedIterator for Iter<'a> {}

impl IntoIterator for Pixel {
    type IntoIter = IntoIter;
    type Item = u16;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {pixel: self, curr: 0}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Pixel::from([10_u16, 20_u16, 30_u16, 40_u16]),
            Pixel::new(10_u16, 20_u16, 30_u16, 40_u16));
    }
}