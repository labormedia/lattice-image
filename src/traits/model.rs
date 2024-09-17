//!   Describes the structure of a Model, which has an Objective type to optimize,
//!   an exchange network function to describe the behaviour of the exchange evaluation,
//!   and and objective function which evaluates two points and returns the type of the Objective.
pub trait Model {
    type Objective;
    type Coefficients;
    fn exchange_network(&mut self, c: Self::Coefficients) -> &mut Self;
    fn optimal_objective(&self, x: (u32, u32)) -> Self::Objective;
}