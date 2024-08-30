use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
        LatticeElement,
    }
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (100,100);
    let mut matrix: MatrixImage<LatticeElement<u32>> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    let mut recipient_matrix = matrix.clone();
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let value = LatticeElement(rng.gen_range(0_u32..u32::MAX));
            let edit_point = (point_x as u32, point_y as u32);
            let _ = matrix.edit_point(edit_point, value);
            #[cfg(debug_assertions)]
            println!("edit_point {edit_point:?} value {value}");
        }
    }
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let center = (point_x as u32,point_y as u32);
            let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
            let mut sum = LatticeElement(0);
            for hood_point in &neighborhood {
                let value = matrix.get_point_value(*hood_point)?;
                sum = sum + value;
                #[cfg(debug_assertions)]
                println!("center {center:?} hood_point {hood_point:?} value {value} sum {sum}");
            }
            let hood_size = LatticeElement(neighborhood.len() as u32);
            let average = (sum / hood_size) * LatticeElement(u32::MAX);
            #[cfg(debug_assertions)]
            println!("sum {sum} hood_size {hood_size} average {average}");
            let _ = recipient_matrix.edit_point(center, average)?;
        }
    }
    
    let center: (u32, u32) = (50,50);
    let _neighborhood = recipient_matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    let _ = recipient_matrix.edit_point(center, 0_u32);
    
    let _image = recipient_matrix
        .draw(Green)?
        .save("matrix.png")?;
        

    #[cfg(debug_assertions)]
    println!("{:?}", _neighborhood);
    
    Ok(())
}