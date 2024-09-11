pub trait Model {
    type Objective;
    fn exchange_network<C>(&self, c: C) -> Self;
    fn objective(&self, x: (u32, u32), y:(u32, u32)) -> Self::Objective;
}