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

    fn foo() {
        let y = 5;
        let z = 100;
    }

    foo();
}
