use crate::{
    MatrixImage,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct FourChannelMatrix<T>
 where T: Clone
{
    height: usize,
    width: usize,
    data: [MatrixImage<T>; 4],
}