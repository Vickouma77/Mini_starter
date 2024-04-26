fn bisection_root<F>(mut f: F, mut a: f64, mut b: f64, eps: f64) -> Option<f64>
where
    F: FnMut(f64) -> f64,
{
    if f(a) * f(b) > 0.0 {
        return None; // No root in this interval
    }

    let mut c = (a + b) / 2.0;
    while (b - a).abs() > eps {
        c = (a + b) / 2.0;
        if f(c) == 0.0 {
            break;
        } else if f(c) * f(a) < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }
    Some(c)
}

fn main() {
    let f = |x: f64| x.powi(3) - 2.0 * x - 5.0;
    let root = bisection_root(f, 1.0, 3.0, 0.0001);
    match root {
        Some(r) => println!("Root: {}", r),
        None => println!("No root found in the given interval."),
    }
}
