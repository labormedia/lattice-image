use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
        LatticeElement,
    },
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let n_sequence = 10;
    let (size_x, size_y) = (100,100);
    let mut matrix: MatrixImage<LatticeElement<u32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let value: LatticeElement<u32> = LatticeElement(rng.gen_range(0_u32..u32::MAX));
            let edit_point = (point_x as u32, point_y as u32);
            let _ = matrix.edit_point(edit_point, value);
            #[cfg(debug_assertions)]
            println!("edit_point {edit_point:?} value {value}");
        }
    }
    
    for id in 0..n_sequence {
        matrix = still_image(matrix)?;
        
        let prepend = "./animation/matrix_".to_owned();
    
        let _image = matrix
            .draw(Green)?
            .save(prepend+&id.to_string()+".png")?;
    }
    
    Ok(())
}

fn still_image(matrix: MatrixImage<LatticeElement<u32>>) -> Result<MatrixImage<LatticeElement<u32>>, Box<dyn Error>> {
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    let mut recipient_matrix = matrix.clone();
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let neighborhood = recipient_matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
            let mut sum = LatticeElement(0);
            for hood_point in &neighborhood {
                let value = recipient_matrix.get_point_value(*hood_point)?;
                sum = sum + value;
                #[cfg(debug_assertions)]
                println!("center {center:?} hood_point {hood_point:?} value {value} sum {sum}");
            }
            let hood_size = LatticeElement(neighborhood.len() as u32);
            let average = sum / hood_size;
            #[cfg(debug_assertions)]
            println!("sum {sum} hood_size {hood_size} average {average}");
            let _ = recipient_matrix.edit_point(center, average)?;
        }
    }
    Ok(recipient_matrix)
}