use std::{env, fs::{File, OpenOptions}, io::{self, Write}, path::Path};
use simple_home_dir::*;
use chrono::Local;

pub fn save(data: u64) {
    let now = Local::now();
    let date_time = now.format("%d-%m-%Y %H:%M:%S").to_string();

    env::set_current_dir(home_dir().unwrap()).unwrap();
    let savef = Path::new("Game_Scores.txt");

    print!("Enter your name:");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    if !savef.exists() {
        let _ = File::create("Math_Scores.txt");
    }

    if let Ok(mut savef) = OpenOptions::new()
    .append(true)
    .open("Math_Scores.txt") 

    {
        let _ = write!(savef, "{}: {} Points, {}\n\n", name.trim(), data, date_time);
    }
}