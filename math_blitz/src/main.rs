use std::{env, io, time::Instant};
use question::question;
use options::{help, bye, trip};
use rand::Rng;
use std::time;

mod question;
mod save;
mod options;

fn main() {
    println!("Welcome, 
Press Enter to Start
Press h for Help
Press e or Ctrl+C to Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "h" => help(),
        "e" => bye(),
        "" => question(),
        _ => trip()
    }
    menu();
}

fn menu() {
    loop {
    println!("\nPress Enter to Play
Press h for Help
Press e to Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "h" => help(),
        "e" => bye(),
        "" => question(),
        _ => trip()
    }
}
}