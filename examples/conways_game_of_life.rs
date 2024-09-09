use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    MatrixImage,
    traits::{
        Matrix,
        Draw,
    },
    error,
};
use rand::Rng;

const ALIVE_VALUE: u8 = 255;
const DEAD_VALUE: u8 = 0;

fn main() -> Result<(), error::MatrixError> {
    let mut rng = rand::thread_rng();
    let n_sequence = 100;
    let (size_x, size_y): (usize, usize) = (100,100);
    let mut matrix = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..(size_x as u32) {
        for point_y in 0..(size_y as u32) {
            let value: u8 = if rng.gen::<u8>() > 128 {
                ALIVE_VALUE
            } else {
                DEAD_VALUE
            };
            let edit_point = (point_x, point_y);
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

fn still_image(matrix: MatrixImage<u8>) -> Result<MatrixImage<u8>, error::MatrixError> {
    let mut new_matrix = MatrixImageBuilder::init().with_height_and_width(matrix.get_width(),matrix.get_height()).build();
    let size_x = matrix.get_height();
    let size_y = matrix.get_width();
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let is_alive = matrix.get_point_value(center)? == ALIVE_VALUE;
            let neighborhood = matrix.get_lattice_neighborhood(center, 1, Neighborhood::Moore);
            let count_hood = count_alive_neighbors(center, neighborhood, &matrix)?;
            #[cfg(debug_assertions)]
            println!("center {center:?} count_hood {count_hood}");
            let new_value = conways_ruleset(is_alive, count_hood);
            let _ = new_matrix.edit_point(center, new_value)?;
        }
    }
    Ok(new_matrix)
}

fn count_alive_neighbors(center: (u32,u32), neighborhood: Vec<(u32,u32)>, matrix: &MatrixImage<u8>) -> Result<u32, error::MatrixError> {
    let center_value = matrix.get_point_value(center)? as u32;
    let mut hood_values = Vec::new();
    for point in neighborhood {
        hood_values.push(matrix.get_point_value(point)?);
    }
    let hood_count = (hood_values.into_iter().map(|x| x as u32).sum::<u32>() - center_value) / ALIVE_VALUE as u32;
    Ok(hood_count)
}

fn conways_ruleset(is_alive: bool, count: u32) -> u8 {
    if is_alive {
        if count < 2 {
            DEAD_VALUE
        } else if count > 3 {
            DEAD_VALUE
        } else {
            ALIVE_VALUE
        }
    } else {
        if count == 3 {
            ALIVE_VALUE
        } else {
            DEAD_VALUE
        }
    }
}