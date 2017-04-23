// Closures as input parameters are possible,
// so return closures as output parameters should
// also be possible. However, retruning closure types
// is problematic because Rust currently only supports
// returning concrete (non-generic) types. Anonymous closure
// types are, by definition, unknown and so returning a closure
// is only possible by making it concrete. This can be done
// via boxing.

// Fn: normal
// FnMut: normal
// FnOnce: unstable

// move keyword must be used, which signals all captures ocurr
// by value.
fn create_fn() -> Box<Fn()> {
  let text = "Fn".to_owned();

  Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
  let text = "FnMut".to_owned();

  Box::new(move || println!("This is a: {}", text))
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();

  fn_plain();
  fn_mut();
}