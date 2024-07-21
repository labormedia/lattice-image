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
            let _ = matrix.edit_point((point_x as u32, point_y as u32), value);
        }
    }
    
    let center = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    
    for point in &neighborhood {
        let _ = matrix.edit_point(*point, 200);
    }
    
    let _ = matrix.edit_point(center, 0);
    
    let _image = matrix
        .draw(Green)?
        .save("matrix.png")?;
        

    
    println!("{:?}", neighborhood);
    
    Ok(())
}
