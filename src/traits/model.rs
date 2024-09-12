//!   Describes the structure of a Model, which has an Objective type to optimize,
//!   an exchange network function to describe the behaviour of the exchange evaluation,
//!   and and objective function which evaluates two points and returns the type of the Objective.
pub trait Model {
    type Objective;
    fn exchange_network<C>(&mut self, c: C) -> &mut Self;
    fn objective(&self, x: (u32, u32), y:(u32, u32)) -> Self::Objective;
}