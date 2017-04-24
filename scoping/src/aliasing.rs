// Data can be borrowed immutably any number of times
// but while immutably borrowed,
// the original data can't be mutably borrowed
struct Point { x: i32, y: i32, z: i32 }

fn main() {
  let mut point = Point { x: 0, y: 0, z: 0 };

  {
    let borrowed_point = &point;
    let another_borrow = &point;

    print!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z);

    // Can't borrow as mutble
  }

  {
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
  }

  let borrowed_point = &point;
}