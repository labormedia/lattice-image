use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Draw,
        LatticeElement,
    },
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n_sequence = 100;
    let (size_x, size_y) = (100,100);
    let mut matrix: MatrixImage<LatticeElement<u32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(u32::MAX - 1_u32))
        .with_height_and_width(size_x,size_y)
        .build();
    
    let center: (u32,u32) = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 10, Neighborhood::Moore);
    
    for point in &neighborhood {
        let _ = matrix.edit_point(*point, u32::MAX/16_u32);
    }
    
    for id in 0..n_sequence {
        
        let prepend = "./animation/matrix_".to_owned();
    
        let _image = matrix
            .draw(Blue)?
            .save(prepend+&id.to_string()+".png")?;
        
        matrix = reaction_diffusion(matrix.clone())?;
    }
    
    Ok(())
}

// Laplace operator.
fn reaction_diffusion(matrix: MatrixImage<LatticeElement<u32>>) -> Result<MatrixImage<LatticeElement<u32>>, Box<dyn Error>> {
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    let mut new_matrix = matrix.clone();
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let new_value = matrix.laplace_operator(center, 1, Neighborhood::VonNeumann)?;
            let _ = new_matrix.edit_point(center, new_value)?;
        }
    }
    Ok(new_matrix)
}