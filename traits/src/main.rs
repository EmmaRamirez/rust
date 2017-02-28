use std::fmt::Debug;

fn main() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    // only defines the type signature
    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }

    struct Rectangle<T> {
        x: T,
        y: T,
        width: T,
        height: T,
    }

    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47
    };

    assert!(r.is_square());

    r.height = 42;
    assert!(r.is_square());

    fn foo<T: Clone + Debug>(x: T) {
        x.clone();
        println!("{:?}", x);
    }

    fn bar<T, K>(x: T, y: K)  where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
    }

}
