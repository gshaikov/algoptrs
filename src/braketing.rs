use crate::Infeasible;

/// Bracket contains a local minimum of the function.
/// Invariant: left < right
pub struct Bracket {
    pub func: fn(f64) -> f64,
    left: f64,
    right: f64,
}

impl Bracket {
    const INIT: f64 = 0.0;
    const GAP: f64 = 1e-2;
    const STEP_FACTOR: f64 = 2.0;
    const MAX_ITER: u32 = 1_000_000;
    /// Given a univariate function, find and return a bracket [left, right]
    /// that encapsulates a local minimum of this function.
    pub fn find(func: fn(f64) -> f64) -> Result<Bracket, Infeasible> {
        let left = Self::INIT;
        let right = Self::INIT + Self::GAP;
        Self::find_with_bracket(
            Bracket { func, left, right },
            if func(left) > func(right) {
                Self::GAP
            } else {
                -Self::GAP
            },
        )
    }

    /// Moving the bracket along the func from left to right (or from right to
    /// left if `step` < 0) until a local minumim appears inside the bracket.
    fn find_with_bracket(mut bracket: Bracket, mut step: f64) -> Result<Bracket, Infeasible> {
        let mut y_right = (bracket.func)(bracket.right);
        for _ in 0..Self::MAX_ITER {
            let right_new = bracket.right + step;
            let y_right_new = (bracket.func)(right_new);
            if y_right_new > y_right {
                if bracket.left < right_new {
                    bracket.right = right_new;
                    return Ok(bracket);
                } else {
                    bracket.right = bracket.left;
                    bracket.left = right_new;
                    return Ok(bracket);
                }
            }
            bracket.left = bracket.right;
            bracket.right = right_new;
            y_right = y_right_new;
            step *= Self::STEP_FACTOR;
        }
        Err(Infeasible(
            "Bracketing algorithm exceeded MAX_ITER iterations",
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_quadratic() {
        let func = |x: f64| (x - 2.0).powi(2);
        let br = Bracket::find(func).unwrap();
        assert!(br.left < br.right);
        assert!(br.left <= 2.0);
        assert!(br.right >= 2.0);
    }

    #[test]
    fn find_error() -> Result<(), String> {
        let func = |x: f64| x;
        let br = Bracket::find(func);
        match br {
            Bracket => Err(format!("Bracket for a funciton without a minimum")),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn find_with_bracket_slope_down() {
        let func = |x: f64| (x - 2.0).powi(2);
        let br = Bracket {
            func,
            left: 0.0,
            right: 0.01,
        };
        let br = Bracket::find_with_bracket(br, 0.01).unwrap();
        assert!(br.left < br.right);
        assert!(br.left <= 2.0);
        assert!(br.right >= 2.0);
    }

    #[test]
    fn find_with_bracket_slope_up() {
        let func = |x: f64| (x + 2.0).powi(2);
        let br = Bracket {
            func,
            left: 0.0,
            right: 0.01,
        };
        let br = Bracket::find_with_bracket(br, -0.01).unwrap();
        assert!(br.left < br.right);
        assert!(br.left <= -2.0);
        assert!(br.right >= -2.0);
    }

    #[test]
    fn find_with_bracket_left_optimal() {
        let func = |x: f64| (x - 2.0).powi(2);
        let br = Bracket {
            func,
            left: 2.0,
            right: 2.01,
        };
        let br = Bracket::find_with_bracket(br, 0.01).unwrap();
        assert!(br.left < br.right);
        assert!(br.left <= 2.0);
        assert!(br.right >= 2.0);
    }

    #[test]
    fn find_with_bracket_right_optimal() {
        let func = |x: f64| (x - 2.0).powi(2);
        let br = Bracket {
            func,
            left: 1.99,
            right: 2.0,
        };
        let br = Bracket::find_with_bracket(br, 0.01).unwrap();
        assert!(br.left < br.right);
        assert!(br.left <= 2.0);
        assert!(br.right >= 2.0);
    }

    #[test]
    fn find_with_bracket_error() -> Result<(), String> {
        let func = |x: f64| x;
        let br = Bracket {
            func,
            left: 0.0,
            right: 0.01,
        };
        let br = Bracket::find_with_bracket(br, 0.01);
        match br {
            Bracket => Err(format!("Bracket for a funciton without a minimum")),
            Err(_) => Ok(()),
        }
    }
}
