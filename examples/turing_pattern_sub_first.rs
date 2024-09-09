use matrix_graph::{
    MatrixImage,
    MatrixImageBuilder,
    Neighborhood,
    traits::{
        Matrix,
        Draw,
        DrawMultiChannel,
        LatticeElement,
    },
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
    let n_sequence = 50000;
    let n_step = 100;
    let (size_x, size_y) = (100,100);
    let mut matrixU: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(1_f32))
        .with_height_and_width(size_x,size_y)
        .build();
    let mut matrixV: MatrixImage<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(0_f32))
        .with_height_and_width(size_x,size_y)
        .build();
    let max_builder: MatrixImageBuilder<LatticeElement<f32>> = MatrixImageBuilder::init()
        .with_initial_value(LatticeElement::from(f32::MAX))
        .with_height_and_width(size_x,size_y);
    
    let coefficients = Coefficients {
        width: size_x as f32,
        height: size_y as f32,
        F: 0.0140,
        k: 0.0540,
        Du: 0.2097,
        Dv: 0.1050,
    };
    
    let centerU: (u32,u32) = (50,50);//(40,55);
    let centerV: (u32,u32) = (50,50);//(60,45);
    let neighborhoodU = matrixU.get_lattice_neighborhood(centerU, 10, Neighborhood::Moore);
    let neighborhoodV = matrixV.get_lattice_neighborhood(centerV, 10, Neighborhood::Moore);
    
    for point in &neighborhoodU {
        let _ = matrixU.edit_point(*point, 0.5_f32);
    }
    for point in &neighborhoodV {
        let _ = matrixV.edit_point(*point, 0.25_f32);
    }
    
    for id in 0..n_sequence {
        if id % n_step == 0 {
            let prepend = "./animation/matrix_".to_owned();
            let matrix_to_drawU = matrixU.clone()*max_builder.build();
            let matrix_to_drawV = matrixV.clone()*max_builder.build();
            let _image = matrixU
                .draw_multi_channel(&[matrix_to_drawU, max_builder.build(), matrix_to_drawV, max_builder.build()], None)?
                .save(prepend+&id.to_string()+".png")?;
        }
        
        (matrixU, matrixV) = reaction_diffusion(matrixU, matrixV, &coefficients)?;
    }
    
    Ok(())
}

fn reaction_diffusion(
    matrixU: MatrixImage<LatticeElement<f32>>, 
    matrixV: MatrixImage<LatticeElement<f32>>,
    c: &Coefficients,
    ) -> Result<(MatrixImage<LatticeElement<f32>>, MatrixImage<LatticeElement<f32>>), error::MatrixError> {
    //let size_x = matrixU.get_width();
    //let size_y = matrixU.get_height();
    let mut new_matrixU = matrixU.clone();
    let mut new_matrixV = matrixV.clone();
    for point_x in 0..c.width as u32 {
        for point_y in 0..c.height as u32 {
            let center = (point_x as u32,point_y as u32);
            let Upoint: f32 = matrixU.get_point_value(center)?.into();
            let lapU: f32 = matrixU.sub_first_laplace_operator(center, 1, Neighborhood::VonNeumann)?.into();
            let Vpoint: f32 = matrixV.get_point_value(center)?.into();
            let lapV: f32 = matrixV.sub_first_laplace_operator(center, 1, Neighborhood::VonNeumann)?.into();
            let dU = (c.Du * lapU) - (Upoint * Vpoint * Vpoint) + (c.F * (1.0 - Upoint) );
            let dV = (c.Dv * lapV) + (Upoint * Vpoint * Vpoint) - ((c.F + c.k) * Vpoint);
            let _ = new_matrixU.edit_point(center, Upoint + dU)?;
            let _ = new_matrixV.edit_point(center, Vpoint + dV)?;
        }
    }
    Ok((new_matrixU, new_matrixV))
}