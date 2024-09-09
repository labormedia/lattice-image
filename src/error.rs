use core::{
    fmt::{
        Display,
        Formatter,
        self,
    },
};
pub use std::error::Error;

#[derive(Debug)]
pub enum MatrixError {
    Overflow,
    //NotImplemented,
}

impl Error for MatrixError {}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Overflow => {
                write!(f, "Overflow Image from Matrix")
            }
        }
    }
}