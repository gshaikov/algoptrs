use algopt::bracketing::bracket::Bracket;
use algopt::bracketing::golden_section;

fn main() {
    let func = |x: f64| f64::exp(x) * f64::sin(x / 4.0 + x * x);
    dbg!(golden_section::search(&Bracket::find(func).unwrap()));
}
