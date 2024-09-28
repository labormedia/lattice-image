use core::ops::Mul;
use crate::MatrixImage;

#[derive(Clone, Debug, PartialEq)]
pub struct NChannelMatrix<T, const N: usize>
 where T: Clone + Mul<Output=T>
{
    height: usize,
    width: usize,
    data: [MatrixImage<T>; N],
}