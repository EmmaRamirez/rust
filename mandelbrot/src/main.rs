extern crate num;
use num::Complex;
use std::str::FromStr;

/**
 * Parse the string `s` as a coordinate par, like "400x600" or "1.0, 0.5"
 * Sepcifically, `s` should have the form <left><sep><right>, where <left> and <right are parsed
 * from T::from_str
 * 
 * If `s` has proper form, reutrn `Some<(x, y)>`. If it doesn't parse correctly, return None
 */
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option< u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}


fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}