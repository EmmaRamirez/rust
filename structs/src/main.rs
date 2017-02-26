

struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Color(i32, i32, i32);

// newtype
fn inches() {
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}

fn black() -> Color {
    let black = Color(0, 0, 0);
    let r = black.0; // access tuple struct via .[pos]
    black
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    black();

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);
}
