use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    // Trait requires fmt with this signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write 'f' strictly
        // will return whether it succeded or failed.
        write!(f, "{}", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    let s = Structure(4);
    println!("{} {}", v, s);
}
