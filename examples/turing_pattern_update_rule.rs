use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Neighborhood,
    traits::{
        self,
        Matrix,
        Draw,
        DrawMultiChannel,
        LatticeElement,
    },
    four_channel::FourChannelMatrix,
    error,
};

/// Struct for the parameter coefficients of the model.
struct Coefficients {
    /// Width of the model lattice.
    width: f32,
    /// Height of the model lattice.
    height: f32,
    /// Feed rate.
    F: f32,
    /// Kill rate.
    k: f32,
    /// Diffusion coefficient for U.
    Du: f32,
    /// Diffusion coefficient for V.
    Dv: f32,
}

fn main() -> Result<(), error::MatrixError> {
    let n_sequence = 7500;
    let n_step = 15;
    let (size_x, size_y) = (100,100);
    let mut matrixU: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(1_f32))
        .with_height_and_width(size_x,size_y)
        .build();
    let mut matrixV: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(0_f32))
        .with_height_and_width(size_x,size_y)
        .build();
    let max_normal_builder: MatrixImageBuilder<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(1_f32))
        .with_height_and_width(size_x,size_y);
    
    let coefficients = Coefficients {
        width: size_x as f32,
        height: size_y as f32,
        F: 0.0460,
        k: 0.0630,
        Du: 0.2097,
        Dv: 0.1050,
    };
    
    let centerU: (u32,u32) = (50,50);
    let centerV: (u32,u32) = (50,50);
    let neighborhoodU = matrixU.get_lattice_neighborhood(centerU, 10, Neighborhood::Moore);
    let neighborhoodV = matrixV.get_lattice_neighborhood(centerV, 10, Neighborhood::Moore);
    
    for point in &neighborhoodU {
        let _ = matrixU.edit_point(*point, 0.25_f32);
    }
    for point in &neighborhoodV {
        let _ = matrixV.edit_point(*point, 0.7_f32);
    }
    let initial_channels = [
        matrixU.clone(), 
        max_normal_builder.build(), 
        matrixV.clone(), 
        max_normal_builder.build()
    ];
    let mut multi_channel: FourChannelMatrix<LatticeElement<f32>> = FourChannelMatrix::from(initial_channels);
    
    for id in 0..n_sequence {
        if id % n_step == 0 {
            let prepend = "./animation/matrix_".to_owned();
            let _image = multi_channel
                .as_normals()
                .multi_channel_image(None)?
                .save(prepend+&id.to_string()+".png")?;
        }
        multi_channel = multi_channel.update_rule_with_coefficients(reaction_diffusion, &coefficients);
    }
    
    Ok(())
}

fn reaction_diffusion(
    multi_channel: &FourChannelMatrix<LatticeElement<f32>>,
    c: &Coefficients,
    ) -> FourChannelMatrix<LatticeElement<f32>> {
    let mut mm = multi_channel.clone();
    for point_x in 0..c.width as u32 {
        for point_y in 0..c.height as u32 {
            let center = (point_x as u32,point_y as u32);
            let Upoint: f32 = mm.get_data_ref()[0].get_point_value(center).unwrap().into();
            let lapU: f32 = mm.get_data_ref()[0].laplace_operator(center, 1, Neighborhood::VonNeumann).unwrap().into();
            let Vpoint: f32 = mm.get_data_ref()[2].get_point_value(center).unwrap().into();
            let lapV: f32 = mm.get_data_ref()[2].laplace_operator(center, 1, Neighborhood::VonNeumann).unwrap().into();
            let dU = (c.Du * lapU) - (Upoint * Vpoint * Vpoint) + (c.F * (1.0 - Upoint) );
            let dV = (c.Dv * lapV) + (Upoint * Vpoint * Vpoint) - ((c.F + c.k) * Vpoint);
            let _ = mm.get_data_mut_ref()[0].edit_point(center, Upoint + dU).unwrap();
            let _ = mm.get_data_mut_ref()[2].edit_point(center, Vpoint + dV).unwrap();
        }
    }
    mm
}