use std::error::Error;
use core::{
    fmt::Debug,
    ops::{
        Div,
        Mul,
        Add,
        Sub,
    },
};
use crate::{
    error::{
        self,
        MatrixError,
    },
    MatrixImage,
    traits::{
        self,
        Matrix,
    },
    Channel,
};
use image::{
    Rgba,
    RgbaImage,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct FourChannelMatrix<T>
 where T: Clone
{
    height: usize,
    width: usize,
    data: [MatrixImage<T>; 4],
}

impl<T: Clone> FourChannelMatrix<T>
 where u8: From<T>
{
    pub fn get_height(&self) -> &usize {
        &self.height
    }
    pub fn get_width(&self) -> &usize {
        &self.width
    }
    pub fn get_data_ref(&self) -> &[MatrixImage<T>; 4] {
        &self.data
    }
    pub fn get_data_mut_ref(&mut self) -> &mut [MatrixImage<T>; 4] {
        &mut self.data
    }
    pub fn update_rule(
        &mut self, 
        rule_function: impl Fn(&Self) -> Self,
    ) -> Self {
        rule_function(self)
    }
    fn check_point_bounds(&self, point: (u32, u32)) -> Result<bool, Box<dyn Error>> {
        if point.0 >= *self.get_width() as u32 || point.1 >= *self.get_height() as u32 { 
            Err(Box::new(error::MatrixError::Overflow))
        } else {
            Ok(true)
        }
    }
    fn into_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), Box<dyn error::Error>> {
        let x: u32 = (absolute_point % self.get_width()) as u32;
        let y: u32 = absolute_point as u32 / *self.get_width() as u32; 
        self.check_point_bounds((x,y))?;
        Ok((x,y))
    }
    pub fn multi_channel_image(&self, channel_order:Option<&[Channel; 4]>) -> Result<RgbaImage, Box<dyn Error>> {
        let mut length_holder = 0_usize;
        let have_same_length = match self.get_data_ref() {
            [head, tail @ ..] => tail.iter().all(|matrix| {
                length_holder = head.get_data().len();  // holds the last length value
                head.get_data().len() == matrix.get_data().len()
            }),
            [_,_,_,_] => false,
        };
        assert!(have_same_length, "Matrices should have the same length.");
        let matrix_order: Vec<MatrixImage<T>> = if let Some(channel_order) = channel_order {
            let mut order = channel_order
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Channel)>>();
            order.sort_by(|indexed_channel_a, indexed_channel_b| {
                    indexed_channel_a.1.cmp(indexed_channel_b.1)
                });
            let ordered_channels: Vec<MatrixImage<T>> = order
                .iter()
                .map(|indexed| indexed.0)
                .map(|index| { self.get_data_ref()[index].clone() })
                .collect();
            ordered_channels
        } else {
            self.get_data_ref().to_vec()
        };
        
        let mut image = RgbaImage::new(self.get_width().clone().try_into()?, self.get_height().clone().try_into()?);

        for i in 0..length_holder {
            let (x,y) = self.into_2d_point(i)?;
            
            let pixel = Rgba([
                u8::from(matrix_order[0].get_data()[i].clone()), 
                u8::from(matrix_order[1].get_data()[i].clone()), 
                u8::from(matrix_order[2].get_data()[i].clone()), 
                u8::from(matrix_order[3].get_data()[i].clone())]);
            image.put_pixel(x, y, pixel);
        }
            
        Ok(image)
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

