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
use crate::Channel;

pub trait Max {
    const MAX: Self;
}

pub trait Draw<T: Debug> where u8: From<T> {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_data_point(&self, point: usize) -> T;
    fn to_2d_point(&self, point: usize) -> Result<(u32, u32), Box<dyn Error>>;
    fn draw(&mut self, color: Channel) -> Result<RgbaImage, Box<dyn Error>> {
        let mut image = RgbaImage::new(self.get_width().try_into()?, self.get_height().try_into()?);
        
        for point in 0..(usize::from(self.get_width()*self.get_height())) {
            let (x,y) = self.to_2d_point(point)?;
            let data_point = self.get_data_point(point);
            println!("data point {:?}", data_point);
            let channel_point = u8::try_from(LatticeElement(200_u32))?;
            
            match color {
                Channel::Red => {
                    image.put_pixel(x, y, Rgba( [channel_point, 0, 0, 1]));
                },
                Channel::Green => {
                    image.put_pixel(x, y, Rgba([0,channel_point, 0, 1]));
                }
                Channel::Blue => {
                    image.put_pixel(x, y, Rgba([0, 0, channel_point, 1]));
                },
                Channel::Alpha => {
                    image.put_pixel(x, y, Rgba([0, 0, 0, channel_point]));
                },
            };
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
pub struct LatticeElement<T: Div + Mul + Add>(pub T);

impl<T: Div<Output = T> + Mul + Add + Sub> Div for LatticeElement<T> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}

impl<T: Div + Mul<Output=T> + Add + Sub> Mul for LatticeElement<T> {
    type Output = Self;
    fn mul(self, value: Self) -> Self {
        Self(self.0 * value.0)
    }
}

impl<T: Div + Mul + Add<Output=T> + Sub> Add for LatticeElement<T> {
    type Output = Self;
    fn add(self, value: Self) -> Self {
        Self(self.0 + value.0)
    }
}

impl<T: Div + Mul + Add + Sub<Output=T>> Sub for LatticeElement<T> {
    type Output = Self;
    fn sub(self, value: Self) -> Self {
        Self(self.0 - value.0)
    }
}

impl<T: Max + Div<Output=T> + Mul<Output=T> + Add<Output=T> + From<u8>> Max for LatticeElement<T> {
    const MAX: Self = LatticeElement(T::MAX);
}

impl<T: Div + Mul + Add> From<T> for LatticeElement<T> {
    fn from(value: T) -> Self {
        LatticeElement(value)
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
        (value.0  / f32::MAX) as u8 * u8::MAX
    }
}

impl From<LatticeElement<u32>> for u8 {
    fn from(value: LatticeElement<u32>) -> Self {
        (value.0  / u32::MAX) as u8 * u8::MAX
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