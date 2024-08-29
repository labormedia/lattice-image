use core::fmt::{
    self,
    Display,
    Debug,
};
use core::ops::{
    Div,
    Mul,
    Add,
    Sub,
};
use std::error::Error;

mod error;
pub mod traits;

use traits::{
    Draw,
    Max,
};

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Channel {
    Red,
    Green,
    Blue,
    Alpha,
}

pub enum Neighborhood {
    VonNeumann,
    Moore,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MatrixImage<T>
 where T: Clone
{
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Clone + Debug> Display for MatrixImage<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data_clone = self.data.clone();
        let mut fmt_accumulator = String::new();
        let mut cursor = 0;
        while cursor < self.height {
            fmt_accumulator += &format!("{} {:?}\n",cursor, &self.data[cursor*self.width..(cursor+1)*self.width]);
            cursor += 1;
        }
        
        write!(f, "{}", fmt_accumulator)
    }
}

impl<T: Mul<Output=T> + Clone> Mul for MatrixImage<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix = self.clone();
        new_matrix.data = new_matrix.data.into_iter().enumerate().map(|(i, value)| { value * rhs.data[i].clone() }).collect();
        new_matrix
    }
}

#[derive(Default)]
pub struct MatrixImageBuilder<T: Clone + Default + traits::Max> {
    initial_value: T,
    template: MatrixImage<T>,
}

impl<T: Clone + Default + traits::Max> MatrixImageBuilder<T> {
    pub fn init() -> Self {
        MatrixImageBuilder::<T>::default().with_initial_value(T::MAX)
    }
    pub fn with_height_and_width(mut self, height: usize, width: usize) -> Self {
        let size: usize = height*width;
        self.template = MatrixImage::<T> {
                height,
                width,
                data: vec![self.initial_value.clone(); size],
            };
        self
    }
    pub fn with_initial_value(mut self, value: T) -> Self {
        self.initial_value = value;
        self
    }
    pub fn build(&self) -> MatrixImage<T> {
        self.template.clone()
    }
}

impl<T: Clone + Debug + Default + traits::Max + Add<Output=T> + Div<Output=T> + Sub<Output=T> + PartialOrd> MatrixImage<T> {
    /// Checks for bounds within the size of the matrix
    fn check_point_bounds(&self, point: (u32, u32)) -> Result<bool, Box<dyn Error>> {
        if point.0 > self.width as u32 || point.1 > self.height as u32 { 
            Err(Box::new(error::MatrixError::Overflow))
        } else {
            Ok(true)
        }
    }
    /// Transforms a 2D point reference point into a 1D point correlated with the matrix raw data and its width/height.
    fn to_absolute_point(&self, point: (u32, u32)) -> Result<usize, Box<dyn Error>> {
        self.check_point_bounds(point)?;
        Ok( (point.0 + point.1 * (self.width as u32)) as usize )
    }
    fn to_2d_point(&self, absolute_point: usize) -> Result<(u32, u32), Box<dyn Error>> {
        let x: u32 = (absolute_point % self.width) as u32;
        let y: u32 = absolute_point as u32 / self.width as u32; 
        self.check_point_bounds((x,y))?;
        Ok((x,y))
    }
    pub fn edit_point<U: Into<u32>>(&mut self, point: (U, U), value: impl Into<T>) -> Result<(), Box<dyn Error>> {
        let absolute_point: usize = self.to_absolute_point((point.0.into(), point.1.into()))?;
        self.data[absolute_point] = value.into();
        Ok(())
    }
    pub fn get_point_value<U: Into<u32>>(&self, point: (U,U)) -> Result<T, Box<dyn Error>>  {
        let absolute_point: usize = self.to_absolute_point((point.0.into(), point.1.into()))?;
        Ok(self.data[absolute_point].clone())
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_lattice_neighborhood<U: Into<i64>>(&self, point: (U, U), distance: usize, hood_type: Neighborhood) -> Vec<(u32, u32)> {
        let distance = distance as i64;
        let (point_x, point_y): (i64, i64) = (point.0.into(), point.1.into());
        let mut point_set = Vec::<(u32,u32)>::new();
        match hood_type {
            Neighborhood::VonNeumann => {
                
                for y_diff in 0..=distance {
                    for x_diff in -y_diff..=y_diff {
                        let mut x_left = (point_x+x_diff) % self.width as i64;
                        let mut y_left = (point_y-distance+y_diff) % self.height as i64;
                        let mut y_right = (point_y+(distance-y_diff)) % self.height as i64;
                        if x_left < 0 {
                            x_left += self.width as i64;
                        }
                        if y_left < 0 {
                            y_left += self.height as i64;
                        }
                        if y_right < 0 {
                            y_right += self.height as i64;
                        }
                        point_set.push((x_left.try_into().expect("Assumes boundary checks."), y_left.try_into().expect("Assumes boundary checks.")));
                        if y_left != y_right {
                            point_set.push((x_left.try_into().expect("Assumes boundary checks."), y_right.try_into().expect("Assumes boundary checks."))); 
                        }
                    };
                }
            },
            Neighborhood::Moore => {
                for y_diff in 0..=2*distance {
                    for x_diff in 0..=2*distance {
                        let mut x_left = (point_x-distance+x_diff) % self.width as i64;
                        let mut y_left = (point_y-distance+y_diff) % self.height as i64;
                        if x_left < 0 {
                            x_left += self.width as i64;
                        }
                        if y_left < 0 {
                            y_left += self.height as i64;
                        }
                        point_set.push((x_left.try_into().unwrap(), y_left.try_into().unwrap())); // TODO: manage overflow error.
                    };
                }
            }
        };
        point_set
    }
    /// T is not bounded to a generic zero value, but to a Default trait implementation,
    /// which is conveniently used to create the sum accumulator later then substracted,
    /// whatever this default value would be.
    /// The returned value is a Tuple with the sum and the length of the neighborhood evaluated.
    pub fn hood_sum(&self, point: (u32, u32), size: usize, hood_type: Neighborhood) -> Result<(T, usize), Box<dyn Error>> {
        let neighborhood = self.get_lattice_neighborhood(point, size, hood_type);
        let mut sum = T::default();  // initial value which is substracted afterwards.
        for hood_point in &neighborhood {
            let value = self.get_point_value(*hood_point)?;
            sum = sum + value;
        };
        Ok(( sum - T::default(), neighborhood.len() ))
    }
    /// Evaluates the Discrete Laplace Operator for the given point coordinates and the size of the neighborhood.
    /// Given that the Neighborhood includes the value of the point being evaluated, we need to substract it from
    /// the neighborhood summation too.
    pub fn laplace_operator(&self, point: (u32, u32), size: usize, hood_type: Neighborhood) -> Result<T, Box<dyn Error>> {
        let point_value = self.get_point_value(point)?;
        let neighborhood = self.get_lattice_neighborhood(point, size, hood_type);
        let mut sum = T::default();  // initial value which is substracted afterwards.
        for hood_point in &neighborhood {
            let hood_point_value = self.get_point_value(*hood_point)?;
            sum = sum + hood_point_value - point_value.clone();
        };
        Ok(sum - T::default())
    }
    /// Receives a point, neighborhood size and Neighborhood type, together with an objective function.
    /// Evaluates all pair of points from the reference to the neighborhood, and returns the point and evaluation T that maximizes
    /// The objective function.
    pub fn optimal_peer(
        &self, self_point: (u32, u32), 
        hood_size: usize, 
        hood_type: Neighborhood, 
        objective: impl Fn(&Self, (u32, u32), (u32, u32)) -> T 
    ) -> Option<((u32, u32), T)> {
        let hood = self.get_lattice_neighborhood(self_point, hood_size, hood_type);
        hood
            .into_iter()
            .map( |neighbor| {
                (neighbor, objective(self, self_point, neighbor))
            })
            .max_by(|a, b| {
                a.1.partial_cmp(&b.1).expect("PartialOrd not implemented for type T.")
            })
    }
}

impl<T: Clone + Default + Debug + Div<Output=T> + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Max + From<u8> + PartialEq + PartialOrd> Draw<T> for MatrixImage<T> 
 where u8: From<T> {
    fn get_width(&self) -> usize {
        self.width    
    }
    fn get_height(&self) -> usize {
        self.height
    }
    fn into_2d_point(&self, point: usize) -> Result<(u32, u32), Box<dyn Error>> {
        self.to_2d_point(point)
    }
    fn into_absolute_point(&self, point: (u32, u32)) -> Result<usize, Box<dyn Error>> {
        self.to_absolute_point(point)
    }
    fn get_data_point(&self, point: usize) -> T {
        self.data[point].clone()
    }
}