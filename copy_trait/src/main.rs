fn main() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);

    let b = true;

    let _x = change_truth(b);
    println!("{}", b);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) ->  bool {
    !x
}
