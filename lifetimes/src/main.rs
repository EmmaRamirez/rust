fn main() {
    /*
        We tell the compiler about the lifetimes of the references
        This is done by making the lifetimes explicit in the function decl.
        descriptive, not prescriptive
        the lifetimes are determined by code, not the annotations, but
        the annotations let the compiler infer
        it can do its own in most cases, but needs dev input in
        complex scenarios
    */
    // 'a = lifetime of a
    // 'b = lifetime of b
    fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {

    }

    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);
        v = skip_prefix(line, p.as_str());
        // goes out of scope but we preserve its lifetimes
    }
    println!("{}", v);

    // we need to ensure Foo cannot outlive the reference to an i32 it contains
    struct Foo<'a> {
        x: &'a i32,
    }

    // we have to declare the lifetime twice in the impl block
    impl<'a> Foo<'a> {
        fn x(&self) -> &'a i32 { self.x }
    }

    fn main() {
        let y = &5; // let _y = 5; let y = &_y;
        let f = Foo { x: y };

        println!("{}", f.x);
    }

    // 'static
    // reference is always alive -- baked into the data
    let g: &'static str = "Hello, world.";

    static FOO: i32 = 5;
    let h: &'static i32 = &FOO;


    /*
        # Lifetime Elisons

        Rusts supports powerful local type inference in the bodies of function but not in their
        signature terms. It's forbidden to allow reasoninga bout types based on their item
        signature alone. However, for ergonomic reasons, a very restricted secondary inference
        algorithm called "lifetime elision" does apply when judging lifetimes. Lifetime elision
        is concerned solely with inferring lifetime parameters using three easily memorizable
        and unambiguous rules.

        input lifetime = associated with paramater of a function
        output lifetime = associated with the retun value of the lifeitme

        ### 3 rules
        - Each elided lifetime in a function's arguments becomes a distinct lifetime parameter.
        - If there is exactly one input lifetime, elided or not, that lifetime is assigned
        to all elided lifetimes in the return values of that function.
        - If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime
        of self is assigned to all elided output lifetime


    */

    fn print(s: &str); // elided
    fn print<'a>(s: &'a str); // expanded

    // Only references (&) need lifetimes, or those related ot lifetimes such as structs.
}
