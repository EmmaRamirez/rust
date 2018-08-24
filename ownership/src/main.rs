fn main() {
    println!("Hello, world!");
}

fn vector_example () {
    let mut v = Vec::new();

    for i in 101...106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105")

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // What's left of our vector
    assert_eq!(v, vec!["101", "104", "substitute"]);
}