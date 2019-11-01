use algopt::bracketing;

fn main() {
    let func = |x: f64| f64::exp(x) * f64::sin(x / 4.0 + x * x);
    dbg!(bracketing::golden_section_search(func));
}
