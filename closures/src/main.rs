fn main() {
    /*
        Sometimes it is useful to wrap up a function and free
        variables for better clarity and reuse. The free variables
        that can be used come from the enclosing scope and are closed
        over when used in the function.

        From this, we get the name 'closures' and Rust provides
        a really great implementation of them.
    */

    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;

        result
    };

    assert_eq!(4, plus_two(2));

    let num = 5;
    {
        let plus_num = |x: i32| x + num;
    }

    let y = &mut num;

}
