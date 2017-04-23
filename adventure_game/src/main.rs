use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;



struct Player {
    hp: i32
}

impl Player {
    pub fn new(hp: i32) -> Player {
        Player {
            hp: hp,
        }
    }
    pub fn hp(&self) -> &i32 { &self.hp }
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
    println!("{}", create_screen("1. Continue "));
    let player = Player::new(100);
    println!("Player HP: {:?}", player.hp);
    // println!("
    //         What class will you play as?
    //         1. Warrior
    //         2. Mage
    //         3. Knight
    // ");

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
