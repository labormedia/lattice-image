use image::{RgbImage, Rgb};
use std::error::Error;

mod error;

pub enum Channel {
    Red,
    Blue,
    Green,
    Alpha,
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
    /// Transforms a 2D point reference point into a 1D point correlated with the matrix raw data and its width/height.
    fn to_absolute_point(&self, point: (u32, u32)) -> Result<usize, Box<dyn Error>> {
        if point.0 > self.width as u32 || point.1 > self.height as u32 { return Err(Box::new(error::MatrixError::Overflow)); };
        Ok( (point.0 + point.1 * (self.width as u32)) as usize )
    }
    fn to_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), Box<dyn Error>> {
        let x: u32 = (absolute_point % self.width) as u32;
        let y: u32 = absolute_point as u32 / self.width as u32; 
        if x > self.width as u32 || y > self.height as u32 { return Err(Box::new(error::MatrixError::Overflow)); };
        Ok((x,y))
    }
    pub fn edit_point(&mut self, point: (u32, u32), value: u8) -> Result<(), Box<dyn Error>> {
        let absolute_point: usize = self.to_absolute_point(point)?;
        self.data[absolute_point] = value;
        Ok(())
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