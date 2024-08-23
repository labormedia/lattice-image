use std::error::Error;
use core::fmt::Debug;
use core::ops::{
    Div,
    Mul,
    Add,
    Sub,
};
use image::{
    Rgba,
    RgbaImage,
};
use crate::{
    MatrixImage,
    Channel,
};

pub trait Max {
    const MAX: Self;
}

pub trait Draw<T: Debug +  Clone + PartialEq> where u8: From<T> {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_data_point(&self, point: usize) -> T;
    fn to_2d_point(&self, point: usize) -> Result<(u32, u32), Box<dyn Error>>;
    fn draw(&self, color: Channel) -> Result<RgbaImage, Box<dyn Error>> {
        let mut image = RgbaImage::new(self.get_width().try_into()?, self.get_height().try_into()?);
        
        for point in 0..(usize::from(self.get_width()*self.get_height())) {
            let (x,y) = self.to_2d_point(point)?;
            let data_point = self.get_data_point(point);
            let channel_point = u8::try_from(data_point.clone())?;
            
            match color {
                Channel::Red => {
                    image.put_pixel(x, y, Rgba( [channel_point, 0, 0, 255]));
                },
                Channel::Green => {
                    image.put_pixel(x, y, Rgba([0,channel_point, 0, 255]));
                }
                Channel::Blue => {
                    image.put_pixel(x, y, Rgba([0, 0, channel_point, 255]));
                },
                Channel::Alpha => {
                    image.put_pixel(x, y, Rgba([0, 0, 0, channel_point]));
                },
            };
        }
        Ok(image)
    }
    fn draw_multi_channel(&self, channels: &[Option<MatrixImage<T>>; 4], channel_order:Option<&[Channel; 4]>) -> Result<RgbaImage, Box<dyn Error>> {
        let mut length_holder = 0_usize;
        let filtered_channels: Vec<MatrixImage<T>> = channels
            .into_iter()
            .filter_map(|option| option.clone())
            .collect();
        let have_same_length = match filtered_channels.as_slice() {
            [head, tail @ ..] => tail.iter().all(|matrix| {
                length_holder = head.data.len().clone();  // holds the last length value
                head.data.len() == matrix.data.len()
            }),
            [] => false,
        };
        assert!(have_same_length, "Matrices should have the same length.");
        let matrix_order: Vec<Option<MatrixImage<T>>> = if let Some(channel_order) = channel_order {
            let mut order = channel_order
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Channel)>>();
            order.sort_by(|indexed_channel_a, indexed_channel_b| {
                    indexed_channel_a.1.cmp(indexed_channel_b.1)
                });
            let ordered_channels: Vec<Option<MatrixImage<T>>> = order
                .iter()
                .map(|indexed| indexed.0)
                .map(|index| {
                    channels[index].clone()
                })
                .collect();
            ordered_channels
        } else {
            channels
                .iter()
                .map(|channel| channel.clone())
                .collect()
        };
        
        let mut image = RgbaImage::new(self.get_width().try_into()?, self.get_height().try_into()?);

        for i in 0..length_holder {
            let (x,y) = self.to_2d_point(i)?;
            
            let pixel = Rgba([
                if let Some(matrix) = &matrix_order[0] { u8::from(matrix.data[i].clone()) } else { 0_u8 }, 
                if let Some(matrix) = &matrix_order[1] { u8::from(matrix.data[i].clone()) } else { 0_u8 }, 
                if let Some(matrix) = &matrix_order[2] { u8::from(matrix.data[i].clone()) } else { 0_u8 }, 
                if let Some(matrix) = &matrix_order[3] { u8::from(matrix.data[i].clone()) } else { 255_u8 }]);
            image.put_pixel(x, y, pixel);
        }
            
        Ok(image)
    }
}

impl Max for u8 {
    const MAX: u8 = u8::MAX;
}

impl Max for u32 {
    const MAX: u32 = u32::MAX;
}

impl Max for f32 {
    const MAX: f32 = f32::MAX;
}

#[derive(Default, Clone, Debug)]
pub struct LatticeElement<T: Div + Mul + Add + PartialEq>(pub T);

/*impl<T: Div<Output = T> + Mul + Add + Sub> Div for LatticeElement<T> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}*/

impl Div for LatticeElement<u32> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0 )
    }
}

impl Div for LatticeElement<f32> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}

impl<T: Div + Mul<Output=T> + Add + Sub + PartialEq> Mul for LatticeElement<T> {
    type Output = Self;
    fn mul(self, value: Self) -> Self {
        Self(self.0 * value.0)
    }
}

impl<T: Div + Mul + Add<Output=T> + Sub + PartialEq> Add for LatticeElement<T> {
    type Output = Self;
    fn add(self, value: Self) -> Self {
        Self(self.0 + value.0)
    }
}

impl<T: Div + Mul + Add + Sub<Output=T> + PartialEq> Sub for LatticeElement<T> {
    type Output = Self;
    fn sub(self, value: Self) -> Self {
        Self(self.0 - value.0)
    }
}

impl<T: Max + Div<Output=T> + Mul<Output=T> + Add<Output=T> + From<u8> + PartialEq> Max for LatticeElement<T> {
    const MAX: Self = LatticeElement(T::MAX);
}

impl<T: Div + Mul + Add + PartialEq> From<T> for LatticeElement<T> {
    fn from(value: T) -> Self {
        LatticeElement(value)
    }
}

impl<T: Div + Mul + Add + PartialEq> PartialEq for LatticeElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<LatticeElement<f32>> for f32 {
    fn from(value: LatticeElement<f32>) -> Self {
       value.0
    }
}

impl From<LatticeElement<u32>> for u32 {
    fn from(value: LatticeElement<u32>) -> Self {
        value.0
    }
}

impl From<LatticeElement<f32>> for u8 {
    fn from(value: LatticeElement<f32>) -> Self {
        let result = ((value.0  / f32::MAX) * (u8::MAX as f32)) as u8;
        result
    }
}

impl From<LatticeElement<u32>> for u8 {
    fn from(value: LatticeElement<u32>) -> Self {
        ((value.0 as f32 / u32::MAX as f32) * u8::MAX as f32) as u8
    }
}

impl From<LatticeElement<u8>> for u8 {
    fn from(value: LatticeElement<u8>) -> Self {
        value.0
    }
}

impl From<u8> for LatticeElement<f32> {
    fn from(value: u8) -> Self {
        LatticeElement(value as f32)
    }
}

impl From<u8> for LatticeElement<u32> {
    fn from(value: u8) -> Self {
        LatticeElement(value as u32)
    }
}