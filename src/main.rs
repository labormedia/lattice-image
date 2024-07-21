use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut matrix = MatrixImageBuilder::init().with_height_and_width(100,100).build();
    let image = matrix
        .draw(Green)?
        .save("matrix.png")?;
        
    let neighborhood = matrix.get_lattice_neighborhood((0,0), 4, Neighborhood::Moore);
    
    println!("{:?}", neighborhood);
    
    Ok(())
}
