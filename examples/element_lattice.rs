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
    let (size_x, size_y): MatrixSize = (100,100);
    let mut matrix = MatrixImageBuilder::<LatticeElement<f32>>::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..(size_x as u32) {
        for point_y in 0..(size_y as u32) {
            let value = rng.gen_range(0_f32..(f32::MAX));
            let _ = matrix.edit_point((point_x, point_y), value);
        }
    }

    let center: Point = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 6, Neighborhood::VonNeumann);
    
    let _ = matrix.edit_point(center, LatticeElement(0_f32));
    
    let _image = matrix
        .draw(Blue)?
        .save("matrix.png")?;

    #[cfg(debug_assertions)]
    println!("{:?}", neighborhood);
    
    Ok(())
}