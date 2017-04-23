use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;



struct Player {
    current_hp: i32,
    total_hp: i32,
}

impl Player {
    pub fn new(current_hp: i32, total_hp: i32) -> Player {
        Player {
            current_hp: current_hp,
            total_hp: total_hp
        }
    }
    pub fn modify_current_hp(amount: i32, &self) -> Player {
        Player {
            current_hp: amount,
            total_hp: &self.total_hp,
        }
    }
    pub fn current_hp(&self) -> &i32 { &self.current_hp }
    pub fn total_hp(&self) -> &i32 { &self.total_hp }
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
    let mut player = Player::new(100, 100);
    println!("Player HP: {:?} / {:?}", player.current_hp, player.total_hp);
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
