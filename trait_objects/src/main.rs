fn main() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // Static dispatch
    fn do_something<T: Foo>(x: T) {
        x.method();
    }

    let x = 5u8;
    do_something(x);


}
