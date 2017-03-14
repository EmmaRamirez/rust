// Fn: by reference &T
// FnMut: by mutable reference &mut T
// FnOnce: by value T

fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;
    // Fn:
    let greeting = "hello";
    // FnMut:
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzz");
        // FnOnce:
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
