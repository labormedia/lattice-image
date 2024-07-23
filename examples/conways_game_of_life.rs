use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    MatrixImage,
};
use std::error::Error;
use rand::Rng;

const ALIVE_VALUE: u8 = 255;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let n_sequence = 10;
    let (size_x, size_y) = (10,10);
    let mut matrix = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let value: u8 = if rng.gen::<u8>() > 128 {
                ALIVE_VALUE
            } else {
                0
            };
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

fn still_image(mut matrix: MatrixImage) -> Result<MatrixImage, Box<dyn Error>> {
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let neighborhood = matrix.get_lattice_neighborhood(center, 1, Neighborhood::Moore);
            let mut sum: u64 = 0;
            let count_hood = count_alive_neighbors(center, neighborhood, &matrix)?;
            #[cfg(debug_assertions)]
            println!("center {center:?} count_hood {count_hood}");
            let _ = matrix.edit_point(center, ALIVE_VALUE)?;
        }
    }
    Ok(matrix)
}

fn count_alive_neighbors(center: (u32,u32), neighborhood: Vec<(u32,u32)>, matrix: &MatrixImage) -> Result<u32, Box<dyn Error>> {
    let center_value = matrix.get_point_value(center)? as u32;
    let mut hood_values = Vec::new();
    for point in neighborhood {
        hood_values.push(matrix.get_point_value(point)?);
    }
    let hood_count = (hood_values.into_iter().map(|x| x as u32).sum::<u32>() - center_value) / ALIVE_VALUE as u32;
    Ok(hood_count)
}