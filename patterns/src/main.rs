fn main() {

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    let point = Point { x: 2, y: 3};

    match origin {
        Point { x, y } => println!("{}, {}", x, y),
    }

    match point {
        // Destructuring works on any compound data type, like tuples or enums
        Point { x, .. } => println!("x is {}", x),
    }
}
