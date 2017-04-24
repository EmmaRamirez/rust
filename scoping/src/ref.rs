#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
  let c = 'Q';

  // A 'ref' borrow on the left side of an assigment
  // is equal to an '&' borrow on the right side.
  let ref ref_c1 = c;
  let ref_c2 = &c;

  let point = Point { x: 0, y: 0 };

  let _copy_of_x = {
    // `ref_to_x` is a reference to the `x` field of `point`.
    let Point { x: ref ref_to_x, y: _ } = point;

    *ref_to_x
  };

  let mut mutable_point = point;

  {
    let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

    *mut_ref_to_y = 1;
  }

  let mut mutable_tuple = (Box::new(5u32), 3u32);

  {
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;
  }
}