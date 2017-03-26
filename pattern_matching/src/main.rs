fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teeen."),
        _ => println!("Ain't special");
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => print!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is ood"),
        _ => println!("No correlation..."),
    }

    // A function `age` which returns a `u32`
    fn age() -> u32 {
        15
    }

    fn ageMatch() {
        println!("Tell me what type of person you are.");

        match age() {
            0 => println!("I'm not born yet I guess."),
            n @ 1..20 => println!("I'm the age of {:?}", n),
            n => println!("I'm an old person the age of {:?}", n),
        }
    }
}
