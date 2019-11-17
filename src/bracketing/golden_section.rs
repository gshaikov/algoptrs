use crate::bracketing::bracket::Bracket;
use crate::Point;
use std::f64;

const FN1_FN: f64 = 0.618_033_988_749_894_9;
const FN2_FN: f64 = 0.381_966_011_250_105_15;
const MAX_OPT_ITER: u32 = 1_000_000;

pub fn search(bracket: &Bracket) -> Point {
    let (mut left, mut right) = bracket.interval();
    let mut right_new = FN1_FN * right + FN2_FN * left;
    let mut y_right_new = (bracket.func)(right_new);
    for _ in 0..MAX_OPT_ITER {
        if f64::abs(left - right) < f64::EPSILON {
            break;
        }
        let left_new = FN1_FN * left + FN2_FN * right;
        let y_left_new = (bracket.func)(left_new);
        if y_left_new < y_right_new {
            right = right_new;
            right_new = left_new;
            y_right_new = y_left_new;
        } else {
            left = right;
            right = left_new;
        }
    }
    Point {
        x: right_new,
        y: y_right_new,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::close;
    use std::f64;

    const POOR_PRECISION: f64 = 1e-6;

    #[test]
    fn search_quadratic() {
        let func = |x: f64| (x - 2.0).powi(2);
        let br = Bracket::find(func).unwrap();
        let point = search(&br);
        // error in evaluating f(x) leads to poor precision of x
        assert!(f64::abs(point.x - 2.0) <= POOR_PRECISION);
        assert!(close(point.y, 0.0));
    }

    #[test]
    fn search_complex() {
        let func = |x: f64| f64::exp(x) * f64::sin(x / 4.0 + x * x);
        let br = Bracket::find(func).unwrap();
        let point = search(&br);
        assert!(point.x < -0.1172);
        assert!(point.x > -0.1173);
        assert!(point.y < -0.0138);
        assert!(point.y > -0.0139);
    }
}
