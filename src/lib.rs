use image::{RgbImage, Rgb};
use std::error::Error;

mod error;

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
pub struct MatrixImage {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

#[derive(Default)]
pub struct MatrixImageBuilder {
    template: MatrixImage,
}

impl MatrixImageBuilder {
    pub fn init() -> Self {
        MatrixImageBuilder::default()
    }
    pub fn with_height_and_width(mut self, height: usize, width: usize) -> Self {
        let size: usize = height*width;
        self.template = MatrixImage {
                height,
                width,
                data: vec![255; size].into(),
            };
        self
    }
    pub fn build(&self) -> MatrixImage {
        self.template.clone()
    }
}

impl MatrixImage {
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
    pub fn get_lattice_neighborhood(&self, point: (u32, u32), distance: usize, hood_type: Neighborhood) -> Vec<(u32, u32)> {
        let distance = distance as i64;
        let (point_x, point_y): (i64, i64) = (point.0 as i64, point.1 as i64);
        let mut point_set = Vec::new();
        match hood_type {
            Neighborhood::VonNeumann => {
                
                for mut y_diff in 0..distance {
                    for x_diff in -y_diff..=y_diff {
                        let mut x_left = (point_x+x_diff) % self.width as i64;
                        let mut y_left = (point_y-distance+y_diff) % self.height as i64;
                        if x_left < 0 {
                            x_left = self.width as i64 + x_left;
                        }
                        if y_left < 0 {
                            y_left = self.height as i64 + y_left;
                        }
                        point_set.push((x_left.try_into().unwrap(), y_left.try_into().unwrap())); 
                    };
                }
                
                for mut y_diff in 0..=distance {
                    for x_diff in -y_diff..=y_diff {
                        let mut x_left = (point_x+x_diff) % self.width as i64;
                        let mut y_left = (point_y+(distance-y_diff)) % self.height as i64;
                        if x_left < 0 {
                            x_left = self.width as i64 + x_left;
                        }
                        if y_left < 0 {
                            y_left = self.height as i64 + y_left;
                        }
                        point_set.push((x_left.try_into().unwrap(), y_left.try_into().unwrap())); // TODO: manage overflow error.
                    };
                }
                
                //panic!("{}",error::MatrixError::NotImplemented);  // TODO: implement neighborhood distance for VonNeumann.
                //point_set.push(point);
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
                _ => {}
            };
        }
        Ok(image)
    }
}