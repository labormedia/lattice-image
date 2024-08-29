use core::{
    cmp,
    ops::{
        Div,
        Mul,
        Add,
        Sub,
    },
};
use super::Max;

#[derive(Default, Clone, Debug)]
pub struct LatticeElement<T: Div + Mul + Add + PartialEq + PartialOrd>(pub T);

/*impl<T: Div<Output = T> + Mul + Add + Sub> Div for LatticeElement<T> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}*/

impl Div for LatticeElement<u32> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0 )
    }
}

impl Div for LatticeElement<f32> {
    type Output = Self;
    fn div(self, value: Self) -> Self {
        Self(self.0 / value.0)
    }
}

impl<T: Div + Mul<Output=T> + Add + Sub + PartialEq + PartialOrd> Mul for LatticeElement<T> {
    type Output = Self;
    fn mul(self, value: Self) -> Self {
        Self(self.0 * value.0)
    }
}

impl<T: Div + Mul + Add<Output=T> + Sub + PartialEq + PartialOrd> Add for LatticeElement<T> {
    type Output = Self;
    fn add(self, value: Self) -> Self {
        Self(self.0 + value.0)
    }
}

impl<T: Div + Mul + Add + Sub<Output=T> + PartialEq + PartialOrd> Sub for LatticeElement<T> {
    type Output = Self;
    fn sub(self, value: Self) -> Self {
        Self(self.0 - value.0)
    }
}

impl<T: Max + Div<Output=T> + Mul<Output=T> + Add<Output=T> + From<u8> + PartialEq + PartialOrd> Max for LatticeElement<T> {
    const MAX: Self = LatticeElement(T::MAX);
}

impl<T: Div + Mul + Add + PartialEq + PartialOrd> From<T> for LatticeElement<T> {
    fn from(value: T) -> Self {
        LatticeElement(value)
    }
}

impl<T: Div + Mul + Add + PartialEq + PartialOrd> PartialEq for LatticeElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Div + Mul + Add + PartialEq + PartialOrd> PartialOrd for LatticeElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}