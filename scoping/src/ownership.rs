// Because variables are in charge of freeing their own resources,
// resources can only have one owner. This also prevents resources
// from being freed more than once. Note that all variables own
// resources (e.g. references).

// When doing assignments (let x = y) or passing function arguments by value,
// the ownership of the resources is transferred. In Rust, this is known
// as 'move'.

// After moving resources, the previous owner can no longer be used.
// This avoids creating dangling pointers.

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
  println!("Destroying a box that contins {}", c);

  // C is destroyed here.
}

fn main() {
  // Stack allocated int
  let x = 5u32;

  // *Copy* x into y
  let y = x;

  println!("x is {}, and y is {}", x, y);

  // a is a pointer to a _heap_ allocated integer
  let a = Box::new(5i32);

  println!("a contains: {}", a);

  // *Move* a into b
  let b = a;

  // println!("a contains: {}", a);
  // ^ Casuses an Error. `a` no longer owns the data

  // Takes ownership of b
  destroy_box(b);

  // println!("b contains: {}", b);
  // ^ Causes an Error.
}