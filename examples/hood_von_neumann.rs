use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
    },
    error,
};

fn main() -> Result<(), error::MatrixError> {
    let mut matrix = MatrixImageBuilder::<u8>::init().with_height_and_width(100,100).build();
    let center: (u32, u32) = (50,50);
    let neighborhood = matrix.get_lattice_neighborhood(center, 3, Neighborhood::VonNeumann);
    
    for point in &neighborhood {
        let _ = matrix.edit_point(*point, 200);
    }
    
    let _ = matrix.edit_point(center, 0);
    
    let _image = matrix
        .draw(Alpha)?
        .save("matrix.png")?;
        

    
    println!("{:?}", neighborhood);
    
    Ok(())
}
