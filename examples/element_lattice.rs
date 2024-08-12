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

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let (size_x, size_y): (usize, usize) = (100,100);
    let mut matrix = MatrixImageBuilder::<LatticeElement<f32>>::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let value: LatticeElement<f32> = LatticeElement(rng.gen()); // MatrixImage<LatticeElement<f32>>
            let _ = matrix.edit_point((point_x as u32, point_y as u32), value);
        }
    }
    
    let center: (u32, u32) = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    
    for point in &neighborhood {
        let _ = matrix.edit_point(*point, 400.0/2.0);
    }
    
    let _ = matrix.edit_point(center, LatticeElement(0_f32));
    
    let _image = matrix
        .draw(Alpha)?
        .save("matrix.png")?;
        

    
    println!("{:?}", neighborhood);
    
    Ok(())
}