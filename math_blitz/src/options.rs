use std::{io, process};

pub fn help() {
    println!("1. You will input an natural number of seconds.
2. The game will spawn questions until time limit.
3. The math questions will be simple addition, subtraction and multiplication.
4. After time's up, your last question is solvable.
5. The game the output your score.\n
Press Enter to return to Menu.");

let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
}

pub fn bye() {
    println!("\nGoodbye ;)");
    process::exit(0);

}

pub fn trip() {
    println!("Assuming you need help.");
    help();
}