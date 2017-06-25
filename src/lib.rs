#![feature(test)]
#![feature(fused)]

#[macro_use] extern crate nom;
#[macro_use] extern crate error_chain;
extern crate test;

mod pixel;
mod parser;
mod farbfeld;
pub mod error;

pub use self::pixel::Pixel;
pub use self::farbfeld::Farbfeld;


