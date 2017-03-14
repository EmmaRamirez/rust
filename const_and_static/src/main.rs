fn main() {
    // requires annotations
    const N: i32 = 5;

    // statics are not inlined
    static M: i32 = 5;

    static NAME: &'static str = "Steve";

    static mut O: i32 = 5;

    unsafe {
        O += 1;

        println!("N: {}", N);
    }

    // NOTE: Prefer const over static.
    // Const allows for more optimizations--especially in crates
}
