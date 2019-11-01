mod bracket;
mod golden_section;

use crate::{Infeasible, Point};
use bracket::Bracket;

pub fn golden_section_search(func: fn(f64) -> f64) -> Result<Point, Infeasible> {
    let bracket = Bracket::find(func)?;
    let solution = golden_section::search(&bracket);
    Ok(solution)
}
