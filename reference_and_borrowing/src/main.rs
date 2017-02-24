fn main() {
    // don't owrry about fold,
    // immutable reference is borrowed

    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a + b);
    }
    // Borrow two vectors and sum them
    // This kind of borrowing does not allow mutation through borrowed reference
    // &T is a reference
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        s1 + s2
    }

    // references are immutable, like bindings
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);

    //***************************//
    let mut x = 5;
    // this scope is necessary
    {
        let y = &mut x;
        // * signals a mutable reference
        *y += 1;
    }
    println!("{}", x);
}
