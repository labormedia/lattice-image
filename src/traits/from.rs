use super::LatticeElement;

impl From<LatticeElement<f32>> for f32 {
    fn from(value: LatticeElement<f32>) -> Self {
       value.0
    }
}

impl From<LatticeElement<f64>> for f64 {
    fn from(value: LatticeElement<f64>) -> Self {
       value.0
    }
}

impl From<LatticeElement<u32>> for u32 {
    fn from(value: LatticeElement<u32>) -> Self {
        value.0
    }
}

impl From<LatticeElement<f32>> for u8 {
    fn from(value: LatticeElement<f32>) -> Self {
        ((value.0  / f32::MAX) * (u8::MAX as f32)) as u8
    }
}

impl From<LatticeElement<f64>> for u8 {
    fn from(value: LatticeElement<f64>) -> Self {
        ((value.0  / f64::MAX) * (u8::MAX as f64)) as u8
    }
}

impl From<LatticeElement<u32>> for u8 {
    fn from(value: LatticeElement<u32>) -> Self {
        ((value.0 as f32 / u32::MAX as f32) * u8::MAX as f32) as u8
    }
}

impl From<LatticeElement<i32>> for u8 {
    fn from(value: LatticeElement<i32>) -> Self {
        ((value.0 as f32 / i32::MAX as f32) * u8::MAX as f32) as u8
    }
}

impl From<LatticeElement<u8>> for u8 {
    fn from(value: LatticeElement<u8>) -> Self {
        value.0
    }
}

impl From<u8> for LatticeElement<f32> {
    fn from(value: u8) -> Self {
        LatticeElement(value as f32)
    }
}

impl From<u8> for LatticeElement<f64> {
    fn from(value: u8) -> Self {
        LatticeElement(value as f64)
    }
}

impl From<u8> for LatticeElement<u32> {
    fn from(value: u8) -> Self {
        LatticeElement(value as u32)
    }
}

impl From<u8> for LatticeElement<i32> {
    fn from(value: u8) -> Self {
        LatticeElement(value as i32)
    }
}