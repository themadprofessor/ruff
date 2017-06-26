//! Ruff is a library for parsing and writing Farbfeld files. Farbfeld is a simple image format
//! defined by [suckless](https://suckless.org). This library follows the
//! [spec](https://http://tools.suckless.org/farbfeld/) for parsing and writing Farbfeld files.

#![feature(test)]
#![feature(fused)]

#[cfg(feature = "serde")]
#[macro_use] extern crate serde;
#[macro_use] extern crate nom;
#[macro_use] extern crate error_chain;
extern crate byteorder;
extern crate test;

mod parser;
mod farbfeld;
pub mod error;
pub mod pixel;

pub use self::pixel::Pixel;
pub use self::farbfeld::Farbfeld;


