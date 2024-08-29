mod error;
pub mod traits;
pub mod four_channel;
pub mod matrix_image;
pub use matrix_image::{
    MatrixImageBuilder,
    MatrixImage,
};

use traits::{
    Draw,
    Max,
    Optimal,
};

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Channel {
    Red,
    Green,
    Blue,
    Alpha,
}

pub enum Neighborhood {
    VonNeumann,
    Moore,
}