// Variables in Rust do more than just hold data in the stack:
// they also own resources, e.g. Box<T> owns memory in the heap.
// Rust enforces RAII, so whenever an object goes out of scope,
// its destructor is called and its own resources are freed.

// This behavior shields against resource leak bugs, so you'll 
// never have to manually free memory or worry about memory leaks again.
fn create_box() {
  let _box1 = Box::new(3i32);

  // `_box1` is destroyed here
}

fn main() {
  let _box2 = Box::new(5i32);

  // Nested Scope
  {
    let _box3 = Box::new(4i32);
    // _box3 is destroyed here
  }

  // We don't have to worry about memory here
  for _ in 0u32...1_000 {
    create_box();
  }

  // _`box2` is destroyed here
}