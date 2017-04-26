extern crate colored;
use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::process::Command;
use std::borrow::Cow;
use std::thread::sleep;
use std::time::Duration;


struct Player {
    current_hp: i32,
    total_hp: i32,
    name: String,
    class: i32,
    attack: i32,
    defense: i32,
    magic: i32,
    coins: i32,
}

impl Player {
    pub fn new(current_hp: i32, total_hp: i32, name: String, class: i32) -> Player {
        Player {
            current_hp: current_hp,
            total_hp: total_hp,
            class: class,
            name: name,
            attack: 10,
            defense: 10,
            magic: 10,
            coins: 100,
        }
    }

    pub fn modify_current_hp(&mut self, amount: i32) {
        self.current_hp += amount;
    }

    pub fn current_hp(&self) -> &i32 { &self.current_hp }
    pub fn total_hp(&self) -> &i32 { &self.total_hp }
    pub fn class(&self) -> &i32 { &self.class }
}

struct Monster {
    current_hp: i32,
    total_hp: i32,
    attack: i32,
    species: String,
}

impl Monster {
    pub fn new(current_hp: i32, total_hp: i32, attack: i32, species: String) -> Monster {
        Monster {
            current_hp: current_hp,
            total_hp: total_hp,
            attack: attack,
            species: species
        }
    }
}

struct Inventory (Vec<Item>);

impl Inventory {
    pub fn new(vec: Vec<Item>) -> Inventory {
        Inventory (vec)
    }

    pub fn contains(&mut self, itemName: String) {
        // if self.0.contains(item) {

        // }
    }

    pub fn _use(&mut self, itemName: String) {

    }

    pub fn add(&mut self, item: Item) {
        self.0.push(item);
    }

    pub fn display(self) {
        println!("
                Bag
        ,.,.,.,,
        ;^.    ;^.  Light Snack     x3
        | |^^^^^^|  Red Gem         x4
        | |      |  Black Gem       x5
        | |      |  Moon Stone      x1
        | |      |  Plain Rock      x1
        '.|      |  
          `------`
        ")
    }
}

struct Item {
    name: String,
    quantity: i32,
}

impl Item {
    pub fn new(name: String) -> Item {
        Item {
            name: name,
            quantity: 0
        }
    }
}

struct Prompt {
    text: String,
    show_vertical_bars: bool
}

impl Prompt {
    pub fn new(text: String, show_vertical_bars: bool) -> Prompt {
        Prompt {
            text: text,
            show_vertical_bars: show_vertical_bars,
        }
    }

    // pub fn to_ui(&self) -> String {
    //
    // }
}

fn create_screen(text: &str) -> String {
    let mut output = String::new();
    output.push_str("+-----------------------------------+\n");
    output.push_str(text);
    output.push_str("\n+-----------------------------------+");
    output
}

fn quit() {
    println!("This will quit the game {} saving. Are you sure? [y/n]", "without".red());
    
}

fn do_nothing() {

}

fn explore() {
    println!("You decided to explore.");
}

fn stats(player: &Player) {
    println!("
        +---------------------------+
               Name: {}            
               HP: {}/{}           
               Atk: {},            
               Def: {},            
               Mag: {}             
        +---------------------------+
    ",
        player.name,
        player.current_hp,
        player.total_hp,
        player.attack,
        player.defense,
        player.magic,
    );
}

fn option(num: i32) -> ColoredString {
    let s = format!("[{}]", num);
    s.yellow()
}

fn init() {
    println!("
        {}{}
        {} New Game
        {} Load Game
        {} Help
    ",
        ">".white(), 
        " Select one option".blue(),
        option(1),
        "[2]".yellow(),
        "[3]".yellow()
    );
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("1 - 3");
    let decision: i32 = match decision.trim().parse() {
        Ok(num) => num,
        Err(num) => panic!(""),
    };
    match decision {
        1 => new_game(),
        2 => do_nothing(),
        3 => help(),
        n => do_nothing(),
    }
}

fn help() {

}

fn new_game() {

}

fn delay() {
    sleep(Duration::new(1, 0));
}

fn pause() {
    // from https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/3
    // let mut stdin = io::stdin();
    // let mut stdout = io::stdout();

    // write!(stdout, "\n> Press [ENTER] to continue...").unwrap();
    // stdout.flush();
}

fn declare_class(class: i32, player: &mut Player) {
    let chosen_class: &'static str = match class {
        1 => {
            player.attack += 5;
            player.magic -= 5;
            "Warrior"
        },
        2 => {
            player.magic += 5;
            player.defense -= 5;
            "Mage"
        },
        3 => {
            player.defense += 5;
            player.attack -= 5;
            "Knight"
        },
        n => "Human"
    };
    println!("Ah! So you're a {}...", chosen_class.blue());
}

fn main() {
    //String::from(playerString.trim())
    // playerCLass
    let mut player = Player::new(100, 100, "".to_string(), 0);
    let mut inventory = Inventory::new(Vec::new());

    init();
    
    

    println!("
                            [][][] /^^/ [][][]
                             |::| /___/ |::|
                             |[]|_|::::|_|[]|
                             |::::::__::::::|
                             |:::::/||/:::::|
                             |:#:::||||::#::|
                            #%*###&*##&*&#*&##
                           ##%%*####*%%%###*%*#
        🏰🏰🏰🏰🏰🏰           {}            🏰🏰🏰🏰🏰🏰
        You are a new hero in the Kingdom of Rust. You will meet
        many friends and fight against many foes. But first, I must
        ask...
    ", "ADVENTURE".yellow());

    

    println!("What is your name?");

    let mut player_string = String::new();

    io::stdin().read_line(&mut player_string)
        .expect("Oops! There was an error reading your input.");

    player.name = String::from(player_string.trim());
    println!("{}, hmmmm...what an interesting name!", player.name.blue());
    

    println!("
        What class will you play as?
        {0} Warrior   ({3}/{4})
        {1} Mage      ({5}/{6})
        {2} Knight    ({7}/{8})
    ",
        option(1),
        option(2),
        option(3),
        "+Atk".blue(),
        "-Mag".red(),
        "+Mag".blue(),
        "-Def".red(),
        "+Def".blue(),
        "-Atk".red()
    );

    let mut player_class = String::new();

    io::stdin().read_line(&mut player_class)
        .expect("You should eneter a number between 1 and 3.");

    let player_class: i32 = match player_class.trim().parse() {
        Ok(num) => num,
        Err(err) => panic!(""),
    };

    declare_class(player_class, &mut player);


    println!("...");
    delay();
    println!("...");
    delay();

    println!("It seems you are fit to lead! The path before you maybe harrowing,\nBut I do not fear one bit that you will fight to your fullest.\nBefore you go, however, take these...");


    delay();
    println!("{}", "\n    ＊ obtained Bag! Use it to keep track of your items.".green());
    
    println!("{}", "    ＊ obtained Map! Use it to track your explorations.\n".green());

    delay();
    println!("Now go!");



    println!("
        What shall you do next?
        [1] {}
        [2] {}
        [3] {}
        [4] {}
        [5] {}
        [6] {}
    ", "Explore".green(), "Statistics".cyan(), "Bag".blue(), "Map".blue(), "Rest (Save)".dimmed(), "Quit".dimmed());

    let mut player_decision = String::new();
    
    io::stdin().read_line(&mut player_decision)
        .expect("You should enter a number between 1 and 4");
    
    let player_decision: i32 = match player_decision.trim().parse() {
        Ok(num) => num,
        Err(err) => panic!("")
    };

    match player_decision {
        1 => explore(),
        2 => stats(&mut player),
        3 => inventory.display(),
        n => do_nothing()
    }

    // let mut playerClass = String::new();
    //
    // io::stdin().read_line(&mut playerClass)
    //     .expect("failed to read line");
    //
    // let playerClass: u32 = match playerClass.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };
}
