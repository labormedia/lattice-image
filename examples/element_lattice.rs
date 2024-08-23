use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Draw,
        LatticeElement,
    },
};
use std::error::Error;
use rand::Rng;

type Point = (u32, u32);
type MatrixSize = (usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    rng.gen_range(0_f32..f32::MAX);
    let (size_x, size_y): MatrixSize = (100,100);
    let mut matrix = MatrixImageBuilder::<LatticeElement<f32>>::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..(size_x as u32) {
        for point_y in 0..(size_y as u32) {
            let value = rng.gen::<f32>(); // LatticeElement<f32>
            let _ = matrix.edit_point((point_x, point_y), (value/64.5)*63.2 - 40.0);
        }
    }
    
    let center: Point = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    
    let _ = matrix.edit_point(center, LatticeElement(0_f32));
    
    let _image = matrix
        .draw(Blue)?
        .save("matrix.png")?;
        

    
    println!("{:?}", neighborhood);
    
    Ok(())
}