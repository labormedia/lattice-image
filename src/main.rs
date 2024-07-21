use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut matrix = MatrixImageBuilder::init().with_height_and_width(100,100).build();
    let image = matrix
        .draw(Green)?
        .save("matrix.png")?;
    Ok(())
}
