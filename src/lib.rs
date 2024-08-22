use core::ops::{
    Div,
    Mul,
    Add,
    Sub,
};
use image::{RgbaImage, Rgba};
use std::error::Error;

mod error;
pub mod traits;

pub enum Channel {
    Red,
    Blue,
    Green,
    Alpha,
}

pub enum Neighborhood {
    VonNeumann,
    Moore,
}

#[derive(Default, Clone, Debug)]
pub struct MatrixImage<T>
 where T: Clone
{
    height: usize,
    width: usize,
    data: Vec<T>,
}

#[derive(Default)]
pub struct MatrixImageBuilder<T: Clone + Default> {
    template: MatrixImage<T>,
}

impl<T: Clone + Default + traits::Max> MatrixImageBuilder<T> {
    pub fn init() -> Self {
        MatrixImageBuilder::<T>::default()
    }
    pub fn with_height_and_width(mut self, height: usize, width: usize) -> Self {
        let size: usize = height*width;
        self.template = MatrixImage::<T> {
                height,
                width,
                data: vec![T::MAX; size].into(),
            };
        self
    }
    pub fn build(&self) -> MatrixImage<T> {
        self.template.clone()
    }
}

impl<T: Clone + Default + traits::Max + Add<Output=T> + Sub<Output=T>> MatrixImage<T> {
    /// Checks for bounds within the size of the matrix
    fn check_point_bounds(&self, point: (u32, u32)) -> Result<bool, Box<dyn Error>> {
        if point.0 > self.width as u32 || point.1 > self.height as u32 { 
            return Err(Box::new(error::MatrixError::Overflow)); 
        } else {
            Ok(true)
        }
    }
    /// Transforms a 2D point reference point into a 1D point correlated with the matrix raw data and its width/height.
    fn to_absolute_point(&self, point: (u32, u32)) -> Result<usize, Box<dyn Error>> {
        self.check_point_bounds(point)?;
        Ok( (point.0 + point.1 * (self.width as u32)) as usize )
    }
    fn to_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), Box<dyn Error>> {
        let x: u32 = (absolute_point % self.width) as u32;
        let y: u32 = absolute_point as u32 / self.width as u32; 
        self.check_point_bounds((x,y))?;
        Ok((x,y))
    }
    pub fn edit_point<U: Into<u32>>(&mut self, point: (U, U), value: impl Into<T>) -> Result<(), Box<dyn Error>> {
        let absolute_point: usize = self.to_absolute_point((point.0.into(), point.1.into()))?;
        self.data[absolute_point] = value.into();
        Ok(())
    }
    pub fn get_point_value<U: Into<u32>>(&self, point: (U,U)) -> Result<T, Box<dyn Error>>  {
        let absolute_point: usize = self.to_absolute_point((point.0.into(), point.1.into()))?;
        Ok(self.data[absolute_point].clone())
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_lattice_neighborhood<U: Into<i64>>(&self, point: (U, U), distance: usize, hood_type: Neighborhood) -> Vec<(u32, u32)> {
        let distance = distance as i64;
        let (point_x, point_y): (i64, i64) = (point.0.into(), point.1.into());
        let mut point_set = Vec::<(u32,u32)>::new();
        match hood_type {
            Neighborhood::VonNeumann => {
                
                for y_diff in 0..=distance {
                    for x_diff in -y_diff..=y_diff {
                        let mut x_left = (point_x+x_diff) % self.width as i64;
                        let mut y_left = (point_y-distance+y_diff) % self.height as i64;
                        let mut y_right = (point_y+(distance-y_diff)) % self.height as i64;
                        if x_left < 0 {
                            x_left = self.width as i64 + x_left;
                        }
                        if y_left < 0 {
                            y_left = self.height as i64 + y_left;
                        }
                        if y_right < 0 {
                            y_right = self.height as i64 + y_left;
                        }
                        point_set.push((x_left.try_into().expect("Assumes boundary checks."), y_left.try_into().expect("Assumes boundary checks.")));
                        if y_left != y_right {
                            point_set.push((x_left.try_into().expect("Assumes boundary checks."), y_right.try_into().expect("Assumes boundary checks."))); 
                        }
                    };
                }
            },
            Neighborhood::Moore => {
                for y_diff in 0..=2*distance {
                    for x_diff in 0..=2*distance {
                        let mut x_left = (point_x-distance+x_diff) % self.width as i64;
                        let mut y_left = (point_y-distance+y_diff) % self.height as i64;
                        if x_left < 0 {
                            x_left = self.width as i64 + x_left;
                        }
                        if y_left < 0 {
                            y_left = self.height as i64 + y_left;
                        }
                        point_set.push((x_left.try_into().unwrap(), y_left.try_into().unwrap())); // TODO: manage overflow error.
                    };
                }
            }
        };
        point_set
    }
    /// T is not bounded to a generic zero value, but to a Default trait implementation,
    /// which is conveniently used to create the sum accumulator later then substracted,
    /// whatever this default value would be.
    /// The returned value is a Tuple with the sum and the length of the neighborhood evaluated.
    pub fn hood_sum(&self, point: (u32, u32), size: usize) -> Result<(T, usize), Box<dyn Error>> {
        let neighborhood = self.get_lattice_neighborhood(point, size, Neighborhood::VonNeumann);
        let mut sum = T::default();  // initial value which is substracted afterwards.
        for hood_point in &neighborhood {
            let value = self.get_point_value(*hood_point)?;
            sum = sum + value;
        };
        Ok(( sum - T::default(), neighborhood.len() ))
    }
    /// Evaluates the Discrete Laplace Operator for the given point coordinates and the size of the neighborhood.
    /// Given that the Neighborhood includes the value of the point being evaluated, we need to substract it from
    /// the neighborhood summation too.
    pub fn laplace_operator(&self, point: (u32, u32), size: usize) -> Result<T, Box<dyn Error>> {
        let value = self.get_point_value(point)?;
        let (mut accumulator, length) = self.hood_sum(point, size)?;
        for _ in 0..length {
            accumulator = accumulator - value.clone()
        }
        Ok(accumulator)
    }
}

/*
impl<T: Clone + Default + traits::Max> traits::Draw for MatrixImage<T> 
 where u8: From<T> 
{
    fn draw(&mut self, color: Channel) -> Result<RgbImage, Box<dyn Error>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        
        for point in 0..self.data.len() {
            let (x,y) = self.to_2d_point(point)?;
            
            let channel_point = u8::try_from(self.data[point].clone())?;
            
            match color {
                Channel::Red => {
                    image.put_pixel(x, y, Rgb( [channel_point, 0, 0]));
                },
                Channel::Green => {
                    image.put_pixel(x, y, Rgb([0,channel_point, 0]));
                }
                Channel::Blue => {
                    image.put_pixel(x, y, Rgb([0, 0, channel_point]));
                },
                _ => {} // TODO: Implement Channel::Alpha
            };
        }
        Ok(image)
    }
}
*/

impl<T: Clone + Default + Div<Output=T> + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + traits::Max + From<u8>> traits::Draw for MatrixImage<traits::LatticeElement<T>> 
 where u8: From<traits::LatticeElement<T>> 
{
    fn draw(&mut self, color: Channel) -> Result<RgbaImage, Box<dyn Error>> {
        let mut image = RgbaImage::new(self.width as u32, self.height as u32);
        
        for point in 0..self.data.len() {
            let (x,y) = self.to_2d_point(point)?;
            
            let channel_point = u8::try_from(self.data[point].clone())?;
            
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

/// MatrixImage<u8> is implemented despite the generic typed MatrixImage<T> because of the From<u8> trait implementation.
impl traits::Draw for MatrixImage<u8> {
    fn draw(&mut self, color: Channel) -> Result<RgbaImage, Box<dyn Error>> {
        let mut image = RgbaImage::new(self.width as u32, self.height as u32);
        
        for point in 0..self.data.len() {
            let (x,y) = self.to_2d_point(point)?;
            
            let channel_point = u8::try_from(self.data[point].clone())?;
            
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