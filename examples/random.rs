use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
    },
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let (size_x, size_y): (usize, usize) = (100,100);
    let mut matrix = MatrixImageBuilder::<u8>::init().with_height_and_width(size_x,size_y).with_generator(|| { rand::thread_rng().gen::<u8>() }).build();
    
    let center: (u32, u32) = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    
    for point in &neighborhood {
        let _ = matrix.edit_point(*point, 200);
    }
    
    let _ = matrix.edit_point(center, 0);
    
    let _image = matrix
        .draw(Blue)?
        .save("matrix.png")?;

    #[cfg(debug_assertions)]
    println!("{:?}", neighborhood);
    
    Ok(())
}
