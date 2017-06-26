#![feature(test)]
#![feature(fused)]

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


