extern crate num;

pub mod bracketing;
pub mod differentiate;

#[derive(Debug)]
pub struct Infeasible(&'static str);

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
