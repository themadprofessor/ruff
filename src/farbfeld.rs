use std::path::Path;
use std::io::{Read, BufReader, Write};
use std::fs::File;

use byteorder::{WriteBytesExt, BigEndian};

use pixel::Pixel;
use error::*;
use parser;

/// A Farbfeld image as defined by the [spec](http://tools.suckless.org/farbfeld/) by Suckless.
#[derive(Debug)]
pub struct Farbfeld {
    pixels: Vec<Pixel>,
    width: u32,
    height: u32
}

impl Farbfeld {
    /// Creates a new Farbfeld object, with the given dimensions and pixels.
    ///
    /// # Errors
    /// Returns an [ErrorKind::InvalidFarbfeldDimensions](error/enum.ErrorKind.html) wrapped in an
    /// [Error](error/struct.Error.html) if width * height != pixels.len().
    ///
    /// ```
    /// # use ::ruff::Farbfeld;
    /// assert!(Farbfeld::new(0, 0, Vec::new()).is_ok());
    /// assert!(Farbfeld::new(10, 10, Vec::new()).is_err());
    /// ```
    ///
    pub fn new(width: u32, height: u32, pixels: Vec<Pixel>) -> Result<Farbfeld> {
        if ((width * height) as usize) != pixels.len() {
            Err(Error::from(ErrorKind::InvalidFarbfeldDimensions))
        } else {
            Ok(Farbfeld {
                width,
                height,
                pixels
            })
        }

    }

    /// Parses the file at the given path into a Farbfeld object.
    ///
    /// # Errors
    /// Returns one of the following errors wrapped in an [Error](error/struct.Error.html).
    /// <ul>
    ///     <li><a href="error/enum.ErrorKing.html">ErrorKind::IoError</a> if the file cannot be opened or read, containing the
    ///     error produced by std.</li>
    ///     <li><a href="error/enum.ErrorKind.html">ErrorKind::InvalidFarbfeldDimensions</a>
    ///     if the file's header's specified dimensions multiplied together do not equal the number
    ///     of parsed pixels.</li>
    ///     <li><a href="error/enum.ErrorKind.html">ErrorKind::NomError</a> if something
    ///     went wrong during parsing.</li>
    /// </ul>
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Farbfeld> {
        File::open(path)
            .map_err(|err| Error::from(ErrorKind::IoError(err)))
            .map(BufReader::new)
            .and_then(Farbfeld::from_read)
    }

    /// Parses the entire of the given Read into a Farbfeld object.
    ///
    /// # Errors
    /// Returns one of the following errors wrapped in an [Error](error/struct.Error.html).
    /// <ul>
    ///     <li><a href="error/enum.ErrorKing.html">ErrorKind::IoError</a> if the reader cannot be
    ///     read to the end, containing the error produced by std.</li>
    ///     <li><a href="error/enum.ErrorKind.html">ErrorKind::InvalidFarbfeldDimensions</a>
    ///     if the reader's header's specified dimensions multiplied together do not equal the number
    ///     of parsed pixels.</li>
    ///     <li><a href="error/enum.ErrorKind.html">ErrorKind::NomError</a> if something
    ///     went wrong during parsing.</li>
    /// </ul>
    pub fn from_read<T: Read>(mut read: T) -> Result<Farbfeld> {
        let mut buff = Vec::new();
        read.read_to_end(&mut buff).map_err(ErrorKind::IoError)?;
        parser::i_to_res(parser::parse_farb(&buff))
    }

    /// Returns all the pixels in the image in row-major order.
    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    /// Tries to return the specified row of pixels from the image. The first row is row 0.
    ///
    /// # Errors
    /// Returns none if the specified row is greater than or equal to the image height.
    pub fn row(&self, row: u32) -> Option<&[Pixel]> {
        if row >= self.height {
            None
        } else {
            let offset = (row * self.width) as usize;
            Some(&self.pixels[offset..offset + self.width as usize])
        }
    }

    /// Returns the width of the image. This is defined in the header of the image.
    pub fn width(&self) -> &u32 {
        &self.width
    }

    /// Returns the height of the image. This is defined in the header of the image.
    pub fn height(&self) -> &u32 {
        &self.height
    }

    /// Writes the image to the given write according to the [spec](http://tools.suckless.org/farbfeld/).
    ///
    /// # Errors
    /// <ul>
    ///     <li> Returns an <a href="error/enum.ErrorKind.html">IoError</a> if the write produces an
    ///     std IoError during write.</li>
    /// </ul>
    pub fn save<T: Write>(&self, write: &mut T) -> Result<()> {
        write.write(b"farbfeld")
            .and_then(|_| write.write_u32::<BigEndian>(self.width))
            .and_then(|_| write.write_u32::<BigEndian>(self.height))
            .and_then(|_| {
                for pixel in self.pixels.iter().flat_map(Pixel::iter) {
                    write.write_u16::<BigEndian>(*pixel)?;
                };
                Ok(())
            })
            .map(|_| ())
            .map_err(|err| Error::from(ErrorKind::IoError(err)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    #[bench]
    fn bench_from_file(b: &mut Bencher) {
        b.iter(|| Farbfeld::from_file("test.ff").unwrap())
    }
}