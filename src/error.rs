use core::{
    num::TryFromIntError,
    fmt::{
        Display,
        Formatter,
        self,
    },
};
use image::error::ImageError;

#[derive(Debug)]
pub enum MatrixError {
    Overflow,
    TryFromIntError(TryFromIntError),
    ImageError(ImageError),
}

impl From<TryFromIntError> for MatrixError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl From<ImageError> for MatrixError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Overflow => {
                write!(f, "Overflow Image from Matrix")
            },
            Self::TryFromIntError(e) => {
                write!(f, "TryFromIntError {e}")
            },
            Self::ImageError(e) => {
                write!(f, "ImageError {e}")
            },
        }
    }
}