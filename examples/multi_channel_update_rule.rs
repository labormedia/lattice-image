use core::ops::Mul;
use matrix_graph::{
    MatrixImageBuilder,
    Channel::*,
    Neighborhood,
    traits::{
        self,
        Matrix,
        Draw,
        DrawMultiChannel,
        Max,
    },
    four_channel::FourChannelMatrix,
    error,
};
use rand::Rng;

fn main() -> Result<(), error::MatrixError> {
    let (size_x, size_y): (usize, usize) = (100,100);
    let mut matrix_builder = MatrixImageBuilder::<u8>::init().with_height_and_width(size_x,size_y);
    let initial_channels = [
        matrix_builder.build(), 
        matrix_builder.build(), 
        matrix_builder.build(), 
        matrix_builder.build()
    ];
    let mut multi_channel: FourChannelMatrix<u8> = FourChannelMatrix::from(initial_channels);
    
    for i in 0..100 {
        multi_channel = multi_channel.update_rule(update_rule);
        let _image = multi_channel
            .multi_channel_image(Some(&[Red, Green, Blue, Alpha]))?
            .save("./stills/matrix".to_owned() + &i.to_string() +".png")?;
    }

    Ok(())
}

fn update_rule<T: Clone + Default + Mul<Output=T> + Max + From<u8>>(multi_matrix: &FourChannelMatrix<T>) -> FourChannelMatrix<T> 
 where u8: From<T>
{
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
