use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Neighborhood,
    traits::{
        Matrix,
        LatticeElement,
        Optimal,
    },
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let size = (5,5);
    let mut rng = rand::thread_rng();
    let mut matrix: MatrixImage<u32> = MatrixImageBuilder::init()
        .with_height_and_width(size.0, size.1)
        .build();
        
    let objective = |objective_matrix: &MatrixImage<u32>, x: (u32, u32), y: (u32, u32)| -> u32 {
        objective_matrix.get_point_value(y).unwrap()
    };
        
    for x in 0..size.0 as u32 {
        for y in 0..size.1 as u32 {
            matrix.edit_point((x,y), rng.gen::<u32>())?;
        }
    }
        
        
    for x in 0..size.0 as u32 {
        for y in 0..size.1 as u32 {
            let optimal = matrix.optimal_peer((x,y), 1, Neighborhood::VonNeumann, objective);
            println!("optimal for point {:?} : {:?}", (x,y), optimal);
        }
    }
            
    println!("Matrix :\n{}", matrix);
    //println!("{:?}", matrix);
        
    Ok(())
}