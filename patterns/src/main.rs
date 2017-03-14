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

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck"),
    }

    let mut x = 5;
    match x {
        ref mut r => println!("Got a mutable referencce to {}", r),
    }

    let x = '💅';

    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }
}
