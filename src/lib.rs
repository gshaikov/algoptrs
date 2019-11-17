extern crate num;

pub mod bracketing;
pub mod differentiate;

use std::f64;

#[derive(Debug)]
pub struct Infeasible(&'static str);

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}
