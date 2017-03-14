fn main() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // Static dispatch
    fn do_something<T: Foo>(x: T) {
        x.method();
    }

    let x = 5u8;
    do_something(x);

    fn do_something(x: &Foo) {
        x.method();
    }

    let g = 5u8;
    do_something(&x as &Foo);

    pub struct TraitObject {
        pub data: *mut (),
        pub vtable: *mut (),
    }

    struct FooVtable {
        destructor: fn(*mut ()),
        size: usize,
        align: usize,
        method: fn(*const ()) -> String,
    }

    fn call_method_on_u8(x: *const ()) -> String {
        let byte: &u8 = unsafe { &*(x as *const u8) };
        byte.method()
    }

    static Foo_for_u8_vtable: FooVtable = FooVtable {
        destructor: /* compiler */,
        size: 1,
        align: 1,
        method: call_method_on_u8 as fn(*const ()) -> String,
    };

    /*
        Object Safety

        Not every trait can be used to make a trait object. For example,
        vectors implement Clone, but if we try to make a trait object.

        let v = vec![1, 2, 3];
        let o = &v as &Clone;

        We get an error: cannot convert to a trait object because
        trait `core::clone::Clone` is not object-safe.

        Only traits that are object-safe can be made into trait objects.
        A trait is object-safe if both of these are true:

        - the trait does not require that Self: Sized
        - all of its methods are object-safe

        So what makes a method object-safe? Each method must require that Self: Sized
        or all of the following:

        - must not have any type parameters
        - must not use Self

        All these rules talk about Self. A good intiution is "except in special circumstances,
        if your traits method uses Self, it is not object-safe."

    */

}
