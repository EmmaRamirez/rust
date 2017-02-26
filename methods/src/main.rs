struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    // use self because it uses the struct it implements
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }

    fn mutable_ref(&mut self) {

    }

    fn takes_ownership(self) {

    }
}

// BUIlDER PATTERN
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0 }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
}

fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());

    let d = c.grow(2.0).area();
    println!("{}", d);
}
