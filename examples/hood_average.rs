use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (100,100);
    let mut matrix = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    for point_x in 0..size_x {
        for point_y in 0..size_y {
            let value: u8 = rng.gen();
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
            let mut sum: u64 = 0;
            for hood_point in &neighborhood {
                let value = matrix.get_point_value(*hood_point)? as u64;
                sum += value;
                #[cfg(debug_assertions)]
                println!("center {center:?} hood_point {hood_point:?} value {value} sum {sum}");
            }
            let hood_size = neighborhood.len();
            let average = sum as usize / hood_size;
            #[cfg(debug_assertions)]
            println!("sum {sum} hood_size {hood_size} average {average}");
            let _ = matrix.edit_point(center, average.try_into()?)?;
        }
    }
    
    let center = (50,50);
    let _neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    let _ = matrix.edit_point(center, 0);
    
    let _image = matrix
        .draw(Green)?
        .save("matrix.png")?;
        

    #[cfg(debug_assertions)]
    println!("{:?}", _neighborhood);
    
    Ok(())
}