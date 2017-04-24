// When data is mutably borrowed, it also freezes
fn main() {
  let mut _mutable_integer = 7i32;

  {
    let _large_integer = &_mutable_integer;

    // Frozen in this scope
    //_mutable_integer = 50;
  }

  // Not frozen here
  _mutable_integer = 3;

}