use std::error::Error;
use core::ops::{
    Div,
    Mul,
};
use image::RgbImage;
use crate::Channel;

pub trait Max {
    const MAX: Self;
}

pub trait Draw {
    fn draw(&mut self, color: Channel) -> Result<RgbImage, Box<dyn Error>>;
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

#[derive(Default, Clone)]
pub struct LatticeElement<T: Div + Mul>(pub T);

impl<T: Div<Output = T> + Mul> Div for LatticeElement<T> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}

impl<T: Div + Mul<Output=T>> Mul for LatticeElement<T> {
    type Output = Self;
    fn mul(self, value: Self) -> Self {
        Self(self.0 * value.0)
    }
}

impl Max for LatticeElement<f32> {
    const MAX: Self = LatticeElement(f32::MAX);
}



impl From<LatticeElement<f32>> for u8 {
    // type Error = Box<dyn Error>;
    fn from(value: LatticeElement<f32>) -> Self {
        ( ( LatticeElement(u8::MAX.into()) / LatticeElement(f32::MAX) ) * value ).0 as u8
    }
}

/*

impl LatticeElement<f32> {
    fn trunc(self) -> Self {
        LatticeElement(self.0.trunc())
    }
}

impl<T: Div<Output=T> + Mul<Output=T> + Max + From<u8>> From<LatticeElement<T>> for u8 {
    fn from(value: LatticeElement<T>) -> Self {
        u8::TryFrom( ( LatticeElement(u8::MAX.into() ) / LatticeElement(T::MAX) ) * value ) 
    }
}
*/