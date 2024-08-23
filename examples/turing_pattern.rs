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
use rand::{
    self,
    Rng
};

fn main() -> Result<(), Box<dyn Error>> {
    let n_sequence = 100;
    let (size_x, size_y) = (100,100);
    let mut matrixU: MatrixImage<LatticeElement<u32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    let mut matrixV: MatrixImage<LatticeElement<u32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    //let mut matrix = randomize_matrix(&mut matrixU, u32::MAX);
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
            .draw(Green)?
            .save(prepend+&id.to_string()+".png")?;
        
        matrix = reaction_diffusion(matrix.clone())?;
    }
    
    Ok(())
}

/// Struct for the parameter coefficients of the model.
struct Coefficients {
    /// Width of the model lattice.
    width: f32,
    /// Height of the model lattice.
    height: f32,
    /// Feed rate.
    F: f32,
    /// Kill rate.
    k: f32,
    /// Diffusion coefficient for U.
    Du: f32,
    /// Diffusion coefficient for V.
    Dv: f32,
}

fn reaction_diffusion(mut matrix: MatrixImage<LatticeElement<u32>>) -> Result<MatrixImage<LatticeElement<u32>>, Box<dyn Error>> {
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

fn randomize_matrix(matrix: &mut MatrixImage<LatticeElement<u32>>, max_value: u32) -> MatrixImage<LatticeElement<u32>> {
    let mut rng = rand::thread_rng();
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    for point_x in 0..size_x {
    for point_y in 0..size_y {
            let value: LatticeElement<u32> = LatticeElement(rng.gen_range(0_u32..(max_value)));
            let edit_point = (point_x as u32, point_y as u32);
            let _ = matrix.edit_point(edit_point, value);
        }
    }
    matrix.clone()
}