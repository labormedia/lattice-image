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
    let n_sequence = 10;
    let (size_x, size_y) = (100,100);
    let mut matrixU: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    let mut matrixV: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    let mut matrix = randomize_matrix(&mut matrixU);
    
    for id in 0..n_sequence {
        matrix = reaction_diffusion(matrix)?;
        
        let prepend = "./animation/matrix_".to_owned();
    
        let _image = matrix
            .draw(Green)?
            .save(prepend+&id.to_string()+".png")?;
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

fn reaction_diffusion(mut matrix: MatrixImage<LatticeElement<f32>>) -> Result<MatrixImage<LatticeElement<f32>>, Box<dyn Error>> {
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let (sum, length) = matrix.hood_sum(center, 3, Neighborhood::VonNeumann)?;
            let hood_size = LatticeElement(length as f32);
            let average = sum / hood_size;
            let _ = matrix.edit_point(center, average)?;
        }
    }
    Ok(matrix)
}

fn randomize_matrix(matrix: &mut MatrixImage<LatticeElement<f32>>) -> MatrixImage<LatticeElement<f32>> {
    let mut rng = rand::thread_rng();
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    for point_x in 0..size_x {
    for point_y in 0..size_y {
            let value: LatticeElement<f32> = LatticeElement(rng.gen());
            let edit_point = (point_x as u32, point_y as u32);
            let _ = matrix.edit_point(edit_point, value);
        }
    }
    matrix.clone()
}