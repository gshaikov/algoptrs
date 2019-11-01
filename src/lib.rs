pub mod bracketing;
pub mod differentiate;

#[derive(Debug)]
pub struct Infeasible(&'static str);
