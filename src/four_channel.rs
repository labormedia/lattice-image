use core::{
    fmt::Debug,
    ops::{
        Div,
        Mul,
        Add,
        Sub,
    },
};
use alloc::vec::Vec;
use crate::{
    error::{
        self,
        MatrixError,
    },
    Neighborhood,
    MatrixImage,
    MatrixImageBuilder,
    traits::{
        self,
        Matrix,
        Max,
        Optimal,
    },
    Channel,
};
use image::{
    Rgba,
    RgbaImage,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct FourChannelMatrix<T>
 where T: Clone + Mul<Output=T>
{
    height: usize,
    width: usize,
    data: [MatrixImage<T>; 4],
}

impl<T: Clone + Default + Max + Mul<Output=T>> FourChannelMatrix<T>
 where u8: From<T>
{
    pub fn get_height(&self) -> &usize {
        &self.height
    }
    pub fn get_width(&self) -> &usize {
        &self.width
    }
    pub fn get_data(self) -> [MatrixImage<T>; 4] {
        self.data
    }
    pub fn get_data_ref(&self) -> &[MatrixImage<T>; 4] {
        &self.data
    }
    pub fn get_data_mut_ref(&mut self) -> &mut [MatrixImage<T>; 4] {
        &mut self.data
    }
    pub fn as_normals(&self) -> Self
    {
        let mut mm = self.clone();
        for matrix in mm.get_data_mut_ref() {
            *matrix = matrix.clone()*(MatrixImageBuilder::init()
                .with_initial_value(T::MAX)
                .with_height_and_width(*self.get_width(),*self.get_height())
                .build());
        }
        mm
    }
    pub fn update_rule(
        &mut self, 
        rule_function: impl Fn(&Self) -> Self,
    ) -> Self {
        rule_function(self)
    }
    pub fn update_rule_with_coefficients<U>(
        &mut self, 
        rule_function: impl Fn(&Self, U) -> Self,
        c: U,
    ) -> Self {
        rule_function(self, c)
    }
    fn check_point_bounds(&self, point: (u32, u32)) -> Result<bool, MatrixError> {
        if point.0 >= *self.get_width() as u32 || point.1 >= *self.get_height() as u32 { 
            Err(error::MatrixError::Overflow)
        } else {
            Ok(true)
        }
    }
    fn into_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), MatrixError> {
        let x: u32 = (absolute_point % self.get_width()) as u32;
        let y: u32 = absolute_point as u32 / *self.get_width() as u32; 
        self.check_point_bounds((x,y))?;
        Ok((x,y))
    }
    pub fn multi_channel_image(&self, channel_order:Option<&[Channel; 4]>) -> Result<RgbaImage, MatrixError> {
        let mut length_holder = 0_usize;
        let have_same_length = match self.get_data_ref() {
            [head, tail @ ..] => tail.iter().all(|matrix| {
                length_holder = head.get_data().len();  // holds the last length value
                head.get_data().len() == matrix.get_data().len()
            }),
        };
        assert!(have_same_length, "Matrices should have the same length.");
        let matrix_order: Vec<MatrixImage<T>> = if let Some(channel_order) = channel_order {
            let mut order = channel_order
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Channel)>>();
            order.sort_by(|indexed_channel_a, indexed_channel_b| {
                    indexed_channel_a.1.cmp(indexed_channel_b.1)
                });
            let ordered_channels: Vec<MatrixImage<T>> = order
                .iter()
                .map(|indexed| indexed.0)
                .map(|index| { self.get_data_ref()[index].clone() })
                .collect();
            ordered_channels
        } else {
            self.get_data_ref().to_vec()
        };
        
        let mut image = RgbaImage::new(self.get_width().clone().try_into()?, self.get_height().clone().try_into()?);

        for i in 0..length_holder {
            let (x,y) = self.into_2d_point(i)?;
            
            let pixel = Rgba([
                u8::from(matrix_order[0].get_data()[i].clone()), 
                u8::from(matrix_order[1].get_data()[i].clone()), 
                u8::from(matrix_order[2].get_data()[i].clone()), 
                u8::from(matrix_order[3].get_data()[i].clone())]);
            image.put_pixel(x, y, pixel);
        }
            
        Ok(image)
    }
}

impl<T: Clone + Debug + Default + traits::Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Mul<Output=T> + PartialOrd> From<[MatrixImage<T>; 4]> for FourChannelMatrix<T> {
    fn from(value: [MatrixImage<T>; 4]) -> Self {
        let height = value[0].get_height();
        let width = value[1].get_width();
        let have_same_length = match value {
            [ref head, ref tail @ ..] => tail.iter().all(|matrix| {
                head.get_data().len() == matrix.get_data().len()
                && head.get_width() == matrix.get_width()
                && head.get_height() == matrix.get_height()
            }),
        };
        assert!(have_same_length);
        FourChannelMatrix {
            height,
            width,
            data: value
        }
    }
}

impl<T: Clone + Debug + Default + traits::Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Mul<Output=T> + PartialOrd> Optimal<T> for FourChannelMatrix<T> 
 where u8: From<T>
{
    fn optimal_peer(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32)) -> T 
    ) -> Option<((u32, u32), T)>
    {
        let hood = self.get_data_ref()[0].get_lattice_neighborhood(self_point, hood_size, hood_type);
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor))
            })
            .max_by(|a, b| {
                a.1.partial_cmp(&b.1).expect("PartialOrd not implemented for type T.")
            })
    }
    fn optimal_peer_internal_values<V>(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32)) -> (T,V) 
    ) -> Option<((u32, u32), (T,V))>
    {
        let hood = self.get_data_ref()[0].get_lattice_neighborhood(self_point, hood_size, hood_type);
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor))
            })
            .max_by(|a, b| {
                a.1.0.partial_cmp(&b.1.0).expect("PartialOrd not implemented for type T.")
            })
    }
    fn optimal_peer_with_coefficients<U: Copy>(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32), U) -> T,
        c: U,
    ) -> Option<((u32, u32), T)>
    {
        let hood = self.get_data_ref()[0].get_lattice_neighborhood(self_point, hood_size, hood_type);
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor, c))
            })
            .max_by(|a, b| {
                a.1.partial_cmp(&b.1).expect("PartialOrd not implemented for type T.")
            })
    }
    fn optimal_peer_internal_values_with_coefficients<U, V, F>(
        &self, 
        self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: F,
        c: &mut U,
    ) -> Option<((u32, u32), (T, V))>
    where
        F: for<'a> Fn(&'a Self, (u32, u32), (u32, u32), &'a mut U) -> (T, V),
    {
        let hood = self.get_data_ref()[0].get_lattice_neighborhood(self_point, hood_size, hood_type);
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor, c))
            })
            .max_by(move |a, b| {
                a.1.0.partial_cmp(&b.1.0).expect("PartialOrd not implemented for type T.")
            })
    }
    fn optimal_peer_internal_values_with_coefficients_and_hood<U, V, F>(
        &self, 
        self_point: (u32, u32), 
        hood: Vec<(u32, u32)>, 
        objective: F,
        c: &mut U,
    ) -> Option<((u32, u32), (T, V))> 
    where 
        F: for<'a> Fn(&'a Self, (u32, u32), (u32, u32), &'a mut U) -> (T, V),
    {
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor, c))
            })
            .max_by(move |a, b| {
                a.1.0.partial_cmp(&b.1.0).expect("PartialOrd not implemented for type T.")
            })
    }
}