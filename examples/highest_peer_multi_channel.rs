use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Neighborhood,
    four_channel::FourChannelMatrix,
    traits::{
        self,
        Matrix,
        LatticeElement,
        Optimal,
    },
};
use std::error::Error;

type Atom = LatticeElement<f32>;

fn main() -> Result<(), Box<dyn Error>> {
    let size = (5,5);
    let mut matrix_channels: MatrixImageBuilder<Atom> = MatrixImageBuilder::init()
        .with_height_and_width(size.0, size.1);
    
    let multi_channel = FourChannelMatrix::from([matrix_channels.build(), matrix_channels.build(), matrix_channels.build(), matrix_channels.build()]);
    let multi_channel_objective = |objective_matrix: &FourChannelMatrix<Atom>, x: (u32, u32), y: (u32, u32)| -> Atom {
        objective_matrix.get_data_ref()[0].get_point_value(y).unwrap()
    };  
        
    for x in 0..size.0 as u32 {
        for y in 0..size.1 as u32 {
            let _ = multi_channel.optimal_peer((x,y), 1, Neighborhood::VonNeumann, multi_channel_objective);
        }
    }
        
    Ok(())
}