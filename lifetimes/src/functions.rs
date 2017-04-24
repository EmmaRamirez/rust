// Ignoring `elision`, function signatures with lifetimes have a few constraints:
// any reference must have an annotated lifetime
// any reference being returned must have the same lifetime as an input or be static

// One input reference with a lifetime 'a' which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
  println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes
fn add_one<'a>(x: &'a mut i32) {
  *x += 1;
}

// Multiple elements with different lifetimes. In this case,
// it would be fine for both to have the same lifetime 'a'
// but in more complex cases, diff lifetimes may be req.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

fn main() {
  let x = 7;
  let y = 9;

  print_one(&x);
  print_multi(&x, &y);

  let z = pass_x(&x, &y);
  print_one(z);

  let mut t = 3;
  add_one(&mut t);
  print_one(&t);
}

