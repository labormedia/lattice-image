use std::error::Error;
use image::RgbImage;
use crate::Channel;

pub trait Max {
    const MAX: Self;
}

pub trait Draw {
    fn draw(&mut self, color: Channel) -> Result<RgbImage, Box<dyn Error>>;
}