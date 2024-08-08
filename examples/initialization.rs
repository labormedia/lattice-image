use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (100,100);
    let matrix: MatrixImage<u32> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    Ok(())
}