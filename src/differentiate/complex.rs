use num::Complex;

const H: f64 = 1e-20;

pub fn complex(f: fn(Complex<f64>) -> Complex<f64>, x: f64) -> f64 {
    f(Complex::new(x, H)).im / H
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::close;
    use std::f64;

    #[test]
    fn diff_complex_sin() {
        let fs = |x: Complex<f64>| x.sin();
        let result = complex(fs, f64::consts::FRAC_PI_3);
        let expected = 0.5;
        assert!(close(result, expected));
    }

    #[test]
    fn diff_complex_square() {
        let fs = |x: Complex<f64>| x * x;
        let result = complex(fs, 4.0);
        let expected = 8.0;
        assert!(close(result, expected));
    }
}
