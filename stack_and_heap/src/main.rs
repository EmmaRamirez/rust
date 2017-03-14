/*
    Rust is a low-level language (duh);
    Stack - fast, memory allocated in Rust by default.
            Allocation is local to function call, limited in size.

    Heap - slower, explicitly allocated, unlimited & global.
            NOT the Heap data structure.
*/
fn main() {
    /*
        Makes a variable binding, allocated to statck by default.
        'stack frame'

        We'll allocate a single 32 bit integer for our stack frame.
        When the function exists, the stack frame gets deallocated.

    */
    let x = 42;


    /*
        THree variables -- y, z, x

    */
    fn foo() {
        let y = 5;
        let z = 100;
    }

    foo();

    /*
        Which to use?
        Stack-allocation uses LIFO semantics for reclaiming storage.
        Heap-allocation is more general, allowing storage at an arbitrary order, with some complexity costs.



    */
}
