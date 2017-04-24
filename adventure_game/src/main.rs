use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::process::Command;
use std::borrow::Cow;


struct Player {
    current_hp: i32,
    total_hp: i32,
    name: String,
    class: i32,
    attack: i32,
    defense: i32,
    magic: i32,
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
            magic: 10
        }
    }
    pub fn modify_current_hp(&self, amount: i32, name: String) -> Player {
        Player {
            current_hp: self.current_hp + amount,
            total_hp: self.total_hp,
            class: self.class,
            name: name,
            attack: self.attack,
            defense: self.defense,
            magic: self.magic,
        }
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


fn main() {
    println!("{}", create_screen("Welcome to Adventure Land!"));
    //println!("{}", create_screen("1. Continue "));
    

    println!("
        You are a new hero in the Kingdom of Rust. You will meet
        many friends and fight against many foes. But first, I must
        ask...
    ");

    println!("What is your name?");

    let mut playerString = String::new();

    io::stdin().read_line(&mut playerString)
        .expect("Oops! There was an error reading your input.");


    println!("Your name shall be {}", playerString);

    println!("
            What class will you play as?
            1> Warrior   (+Atk/-Mag)
            2> Mage      (+Mag/-Def)
            3> Knight    (+Def/-Atk)
    ");

    let mut playerClass = String::new();

    io::stdin().read_line(&mut playerClass)
        .expect("You should eneter a number between 1 and 3.");

    let playerClass: i32 = match playerClass.trim().parse() {
        Ok(num) => num,
        Err(err) => panic!(""),
    };

    print!("You chose: ");
    match playerClass {
        1 => println!("Warrior"),
        2 => println!("Mage"),
        3 => println!("Knight"),
        n => println!("Human")
    }

    let mut player = Player::new(100, 100, String::from(playerString), playerClass);

    println!("
            What shall you do next?
            [1] Explore
            [2] Statistics
            [3] Rest (Save)
            [4] Quit
    ");

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
