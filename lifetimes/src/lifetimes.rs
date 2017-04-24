// A lifetime is a construct the compiler uses to ensure
// all borrows are valid. Specifically, a variables lifetime
// begins when it is created and ends when it is destroyed.

// Lifetimes and scopes are not the same.

fn main() {
  let i = 3; // lifetime starts

  {
    let borrow1 = &i; // `borrow1` lifetime starts

    // borrow1 ends
  }

  {
    let borrow2 = &i; // borrow2 lifetime starts
  }

  // i ends
}