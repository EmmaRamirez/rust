use List::*;

enum List {
    // Const: Tuple struct that wraps an element anda  pointer to next
    Const(u32, Box<List>),
    //Note that signifies end
    Nil,
}

impl List {
    fn new() -> List {
        // Nil has type List
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched because of the behavior
        // of this method
        // `self` has type &List and `*self` has type `List`
        // matchign on a concrete type `T`
        match *self {
            // Cant't take ownership of tail, because `self` is borrowed
            // instead take a reference to the tail
            Const(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Const(head, ref tail) => {
                // format returns a heap alloc string
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length of: {}", list.len());
    println!("{}", list.stringify());
}