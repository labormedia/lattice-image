use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    error,
};

fn main() -> Result<(), error::MatrixError> {
    let (size_x, size_y) = (100,100);
    let _matrix: MatrixImage<u32> = MatrixImageBuilder::init().with_height_and_width(size_x,size_y).build();
    
    Ok(())
}