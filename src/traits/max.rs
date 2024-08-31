pub trait Max {
    const MAX: Self;
}

impl Max for u8 {
    const MAX: u8 = u8::MAX;
}

impl Max for u32 {
    const MAX: u32 = u32::MAX;
}

impl Max for f32 {
    const MAX: f32 = f32::MAX;
}

impl Max for i32 {
    const MAX: i32 = i32::MAX;
}