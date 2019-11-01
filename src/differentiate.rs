use std::f64;

pub fn central(f: impl Fn(f64) -> f64, x: f64) -> f64 {
    // cbrt(eps) is prefered but gives a large error
    let h = f64::cbrt(f64::EPSILON);
    (f(x + h / 2.0) - f(x - h / 2.0)) / h
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_eps() -> f64 {
        f64::cbrt(f64::EPSILON)
    }

    #[test]
    fn diff_central_sin() {
        let fs = |x| f64::sin(x);
        let result = central(fs, f64::consts::FRAC_PI_3);
        let expected = 0.5;
        assert!((result - expected).abs() <= test_eps());
    }

    #[test]
    fn diff_central_square() {
        let fs = |x| x * x;
        let result = central(fs, 4.0);
        let expected = 8.0;
        assert!((result - expected).abs() <= test_eps());
    }
}
