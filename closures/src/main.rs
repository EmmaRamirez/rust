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

    let mut num = 5;

    {
        // Takes ownership
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }

    fn call_with_one<F>(some_closure: F) -> i32
        // Because Fn is a trait, we can use it as a bound for our
        // generic type. In this case, our closure takes an i32
        // as an argument and returns i32, so the generic bound
        // we use Fn(i32) -> i32
        // This code gets monomorphized, and therefore, we'll
        // be doing static dipsatch into the closure.
        // In many languages, closures are inherently heap
        // allocated, and will always involve dynamic dispatch
        // In Rust, we can stack allocate our closure environment
        // and statically dispatch the call.
        where F: Fn(i32) -> i32 {
            some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    fn call_with_one_2(some_closure: &Fn(i32)) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one_2(&|x| x + 2);

}
