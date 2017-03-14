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

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        // NOTE: if you use @ with | you need to bound it at each part of the pattern
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

    enum OptionalInt {
        Value(i32),
        Missing
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Go an int!"),
        OptionalInt::Missing => println!("No such luck"),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}
