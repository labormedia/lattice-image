use image::{RgbImage, Rgb};
use std::error::Error;

mod error;
mod traits;

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

impl MatrixImage<u8> {
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
    pub fn edit_point(&mut self, point: (u32, u32), value: u8) -> Result<(), Box<dyn Error>> {
        let absolute_point: usize = self.to_absolute_point(point)?;
        self.data[absolute_point] = value;
        Ok(())
    }
    pub fn get_point_value(&self, point: (u32,u32)) -> Result<u8, Box<dyn Error>>  {
        let absolute_point: usize = self.to_absolute_point(point)?;
        Ok(self.data[absolute_point])
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_lattice_neighborhood(&self, point: (u32, u32), distance: usize, hood_type: Neighborhood) -> Vec<(u32, u32)> {
        let distance = distance as i64;
        let (point_x, point_y): (i64, i64) = (point.0 as i64, point.1 as i64);
        let mut point_set = Vec::new();
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
                        point_set.push((x_left.try_into().unwrap(), y_left.try_into().unwrap()));
                        if y_left != y_right {
                            point_set.push((x_left.try_into().unwrap(), y_right.try_into().unwrap())); 
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
    
    pub fn draw(&mut self, color: Channel) -> Result<RgbImage, Box<dyn Error>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        
        for point in 0..self.data.len() {
            let (x,y) = self.to_2d_point(point)?;
            
            match color {
                Channel::Red => {
                    image.put_pixel(x, y, Rgb([self.data[point], 0, 0]));
                },
                Channel::Green => {
                    image.put_pixel(x, y, Rgb([0,self.data[point], 0]));
                }
                Channel::Blue => {
                    image.put_pixel(x, y, Rgb([0, 0, self.data[point]]));
                },
                _ => {} // TODO: Implement Channel::Alpha
            };
        }
        Ok(image)
    }
}

impl traits::Max for u8 {
    const MAX: u8 = u8::MAX;
}

impl traits::Max for u32 {
    const MAX: u32 = u32::MAX;
}