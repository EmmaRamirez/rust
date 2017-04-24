// Mutable data can be mutably borrowed using &mut T.
// This is called a mutable reference and gives read/write
// access to the borrower. In contrast, &T borrows the
// data via an immutable reference, and the borrower can read the data
// but not modify it
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  // &'static str is a reference to a string allocated in ROM
  author: &'static str,
  title: &'static str,
  year: u32
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
  println!("I immutably borrowed {} - {}", book.title, book.year);
}

fn new_edition(book: &mut Book) {
  book.year = 2017;
  println!("I mutably borrowed {} - {}", book.title, book.year);
}

fn main() {
  let immutabook = Book {
    author: "Emma Ramirez",
    title: "A Terrible Rust Book",
    year: 2017
  };

  let mut mutabook = immutabook;

  // Immutably borrow immutalbe object
  borrow_book(&immutabook);

  // Immutable borrow a mutable
  borrow_book(&mutabook);

  // Borrow a mutable object as mutable
  new_edition(&mut mutabook);

  // new_edition(&mut immutabook);
  // ^ Causes Error
}