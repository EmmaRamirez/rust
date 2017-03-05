mod my {
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    let white_box = my::WhiteBox { contents: "public info" };

    println!("The white box contains: {}", white_box.contents);

    let _black_box = my::BlackBox::new("classified information");
}
