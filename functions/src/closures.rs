fn main() {
    // increment via closures
    fn function (i: i32) -> i32 { i + 1 }

    // closures are anonymous, here we bind them to references
    // Annotation is identical to function annotation
    let closure_annotated = |i: i32| -> i32 { i + 1 };

    let closure_inferred = |i    | i + 1;

    let i = 1;
    // call function and closures
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an i32
    // the return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one());
}
