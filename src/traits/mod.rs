use std::error::Error;
use core::fmt::Debug;
use core::{
    cmp,
    ops::{
        Div,
        Mul,
        Add,
        Sub,
    },
};
use image::{
    Rgba,
    RgbaImage,
};
use crate::{
    error,
    MatrixImage,
    Channel,
    Neighborhood,
};

pub mod from;
pub use from::*;

pub mod lattice_element;
pub use lattice_element::*;

pub mod max;
pub use max::*;

pub trait Matrix<T>
//where
// T: Clone + Debug + Default + Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + PartialOrd
{
    type Data;
    fn get_data(&self) -> Self::Data;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    /// Checks for bounds within the size of the matrix
    fn check_point_bounds(&self, point: (u32, u32)) -> Result<bool, Box<dyn Error>> {
        if point.0 >= self.get_width() as u32 || point.1 >= self.get_height() as u32 { 
            Err(Box::new(error::MatrixError::Overflow))
        } else {
            Ok(true)
        }
    }
    fn get_point_value<U: Into<u32>>(&self, point: (U,U)) -> Result<T, Box<dyn error::Error>>;
    fn get_absolute_point_data(&self, absolute_point: usize) -> T;
    fn into_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), Box<dyn error::Error>> {
        let x: u32 = (absolute_point % self.get_width()) as u32;
        let y: u32 = absolute_point as u32 / self.get_width() as u32; 
        self.check_point_bounds((x,y))?;
        Ok((x,y))
    }
    /// Transforms a 2D point reference point into a 1D point correlated with the 
    /// matrix raw data and its width/height.
    fn into_absolute_point(&self, point: (u32, u32)) -> Result<usize, Box<dyn Error>> {
        self.check_point_bounds(point)?;
        Ok( (point.0 + point.1 * (self.get_width() as u32)) as usize )   
    }
    fn edit_point<U: Into<u32>>(&mut self, point: (U, U), value: impl Into<T>) -> Result<(), Box<dyn error::Error>>;
}

pub trait Draw<T>: Matrix<T> 
where 
 T: Clone + Debug + Default + Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + PartialOrd,
 u8: From<T> 
{
    fn draw(&self, color: Channel) -> Result<RgbaImage, Box<dyn Error>> {
        let mut image = RgbaImage::new(self.get_width().try_into()?, self.get_height().try_into()?);
        
        for point in 0..(self.get_width()*self.get_height()) {
            let (x,y) = self.into_2d_point(point)?;
            let data_point = self.get_absolute_point_data(point);
            let channel_point = u8::from(data_point.clone());
            
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
}

pub trait DrawMultiChannel<T>: Matrix<T>
 where 
 T: Clone + Debug + Default + Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + PartialOrd,
 u8: From<T> 
{
    fn draw_multi_channel(&self, channels: &[MatrixImage<T>; 4], channel_order:Option<&[Channel; 4]>) -> Result<RgbaImage, Box<dyn Error>> {
        let mut length_holder = 0_usize;
        let have_same_length = match channels.as_slice() {
            [head, tail @ ..] => tail.iter().all(|matrix| {
                length_holder = head.get_data().len();  // holds the last length value
                head.get_data().len() == matrix.get_data().len()
            }),
            [] => false,
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
                .map(|index| { channels[index].clone() })
                .collect();
            ordered_channels
        } else {
            channels.to_vec()
        };
        
        let mut image = RgbaImage::new(self.get_width().try_into()?, self.get_height().try_into()?);

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

pub trait Optimal<T>
where
 T: PartialOrd
{
    /// Receives a point, neighborhood size and Neighborhood type, together with an objective function.
    /// Evaluates all pair of points from the reference to the neighborhood, and returns the point and evaluation T that maximizes
    /// The objective function.
    fn optimal_peer(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32)) -> T 
    ) -> Option<((u32, u32), T)>;
    fn optimal_peer_with_coefficients<U: Copy>(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32), U) -> T,
        c: U,
    ) -> Option<((u32, u32), T)>;
}