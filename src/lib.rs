#![no_std]
#[macro_use]
extern crate alloc;
pub mod error;
pub mod traits;
pub mod four_channel;
pub mod matrix_image;
pub use matrix_image::{
    MatrixImageBuilder,
    MatrixImage,
};

use traits::{
    Matrix,
    Draw,
    DrawMultiChannel,
    Optimal,
};

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Channel {
    Red,
    Green,
    Blue,
    Alpha,
}

#[derive(Clone, Copy)]
pub enum Neighborhood {
    VonNeumann,
    Moore,
}

impl Neighborhood {
    pub fn length(self, size: usize) -> usize {
        match self {
            Self::VonNeumann => {
                size * size + ((1 + size) * (1 + size))
            },
            Self::Moore => {
                3_usize.pow(size as u32) - 1
            },
        }
    }
}