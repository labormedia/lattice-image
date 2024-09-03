use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
        DrawMultiChannel,
    },
    four_channel::FourChannelMatrix,
};
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let (size_x, size_y): (usize, usize) = (100,100);
    let mut matrix = MatrixImageBuilder::<u8>::init().with_height_and_width(size_x,size_y).build();
     
    let mut multi_channel: FourChannelMatrix<u8> = FourChannelMatrix::from([matrix.clone(), matrix.clone(), matrix.clone(), matrix.clone()]);
    
    for i in 0..100 {
        multi_channel = multi_channel.update_rule(update_rule);
        let _image = matrix
            .draw_multi_channel(&multi_channel.get_data_ref(), Some(&[Red, Green, Blue, Alpha]))?
            .save("./stills/matrix".to_owned() + &i.to_string() +".png")?;
    }

    Ok(())
}

fn update_rule<T: Clone + std::convert::From<u8>>(multi_matrix: &FourChannelMatrix<T>) -> FourChannelMatrix<T> {
    let mut mm = multi_matrix.clone();
    let mut rng = rand::thread_rng();
    for point_x in 0..*mm.get_width() as usize {
        for point_y in 0..*mm.get_height() as usize {
            for mut matrix in mm.get_data_mut_ref() {
               let value: u8 = rng.gen();
                let _ = matrix.edit_point((point_x as u32, point_y as u32), value);             
            }
        }
    }
    mm
}
