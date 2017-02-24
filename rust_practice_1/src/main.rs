fn main() {
    let (x, y) = (1, 2);

    let x: i32 = 5;
    {
        let y: i32 = 3;
        println!("{}", y);
    }

    let a = [1, 2, 3];
    let mut m = [1, 2, 3];

    let v: (i32, &str) = (1, "hello");

    for (inde, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}," linenumber, line);
    }

    let mut x = 5;
    let mut done = false;

    while !done {
        x+= x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }

    loop {
        x+= x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }

    print_sum(5, 6);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn vectors() {
    let v = vec![1, 2, 3, 4, 5];
    let v = vec![0; 10];

    for i in &v {
        println!("A reference to {}", i);
    }

    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("sorry, this vector is too short.")
    }
}

fn ownership() {
    fn foo() {
        let v = vec![1, 2, 3];
    }
}
