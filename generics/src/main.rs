fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    // let x: Option<i32> = Some(5);
    // let y: Option<f64> = Some(5.0f64);

    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    fn takes_anything<T>(x: T) {

    }

    struct Point<T> {
        x: T,
        y: T
    }

    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    let int_origin = Point { x: 0, y: 0 };

}
