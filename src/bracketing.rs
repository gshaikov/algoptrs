mod bracket;
mod golden_section;

use crate::Infeasible;
use bracket::Bracket;
use golden_section::Point;

pub fn golden_section_search(func: fn(f64) -> f64) -> Result<Point, Infeasible> {
    let bracket = Bracket::find(func)?;
    let solution = golden_section::search(&bracket);
    Ok(solution)
}
