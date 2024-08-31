use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
        DrawMultiChannel,
        LatticeElement,
    },
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n_sequence = 10;
    let (size_x, size_y) = (100,100);
    let hood_distance = 10;
    let hood_size = Neighborhood::VonNeumann.length(hood_distance) as i32;
    let mut matrixU: MatrixImage<LatticeElement<i32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(i32::MAX/(hood_size)))
        .with_height_and_width(size_x,size_y)
        .build();
    let mut matrixV: MatrixImage<LatticeElement<i32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(i32::MAX/(hood_size)))
        .with_height_and_width(size_x,size_y)
        .build();
    let mut matrix_zeroes: MatrixImage<LatticeElement<i32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(0_i32))
        .with_height_and_width(size_x,size_y)
        .build();
    let mut matrix_max: MatrixImage<LatticeElement<i32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(i32::MAX))
        .with_height_and_width(size_x,size_y)
        .build();
    
    //let mut matrix = randomize_matrix(&mut matrixU, i32::MAX);
    
    let centerU: (u32,u32) = (40,55);
    let centerV: (u32,u32) = (60,45);
    let neighborhoodU = matrixU.get_lattice_neighborhood(centerU, 10, Neighborhood::Moore);
    let neighborhoodV = matrixV.get_lattice_neighborhood(centerV, 10, Neighborhood::Moore);
    
    for point in &neighborhoodU {
        let _ = matrixU.edit_point(*point, 0);
    }
    for point in &neighborhoodV {
        let _ = matrixV.edit_point(*point, 0);
    }
    
    for id in 0..n_sequence {
        
        let prepend = "./animation/matrix_".to_owned();
    
        let _image = matrixU
            .draw_multi_channel(&[matrixU.clone(), matrix_zeroes.clone(), matrixV.clone(), matrix_max.clone()], None)?
            .save(prepend+&id.to_string()+".png")?;
        
        matrixU = laplace_operator(matrixU)?;
        matrixV = laplace_operator(matrixV)?;
    }
    
    Ok(())
}

fn laplace_operator(matrix: MatrixImage<LatticeElement<i32>>) -> Result<MatrixImage<LatticeElement<i32>>, Box<dyn Error>> {
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