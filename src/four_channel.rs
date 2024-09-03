use core::fmt::Debug;
use core::ops::{
    Div,
    Add,
    Sub,
};
use crate::{
    MatrixImage,
    traits::{
        self,
        Matrix,
    },
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct FourChannelMatrix<T>
 where T: Clone
{
    height: usize,
    width: usize,
    data: [MatrixImage<T>; 4],
}

impl<T: Clone> FourChannelMatrix<T> {
    pub fn get_data(&self) -> [MatrixImage<T>; 4] {
        self.data.clone()
    }
    pub fn update_rule(
        mut self, 
        rule_function: impl Fn(Self) -> Self,
    ) -> Self {
        rule_function(self)
    }
}

impl<T: Clone + Debug + Default + traits::Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + PartialOrd> From<[MatrixImage<T>; 4]> for FourChannelMatrix<T> {
    fn from(value: [MatrixImage<T>; 4]) -> Self {
        let mut height = value[0].get_height();
        let mut width = value[1].get_width();
        let have_same_length = match value {
            [ref head, ref tail @ ..] => tail.iter().all(|matrix| {
                head.get_data().len() == matrix.get_data().len()
                && head.get_width() == matrix.get_width()
                && head.get_height() == matrix.get_height()
            }),
        };
        assert!(have_same_length);
        FourChannelMatrix {
            height,
            width,
            data: value
        }
    }
}

