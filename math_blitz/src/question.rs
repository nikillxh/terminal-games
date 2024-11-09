use std::time::Duration;
use std::{io, time::Instant};
use rand::{Rng, seq::SliceRandom};
use std::io::Write;
use std::thread;

use crate::save::save;

pub fn question() {
    let mut timer = 20;
    print!("Enter natural number time: ");
    io::stdout().flush().unwrap();

    let mut timeinp: String = String::new();
    io::stdin().read_line(&mut timeinp);

    match timeinp.trim().parse::<u64>() {
        Ok(parsed_time) if parsed_time > 0 => {
            timer = parsed_time;
            println!("Challenge set to {} seconds.\n", parsed_time);
        }
        _ => {
            println!("Default time set to 20 seconds.\n");
        }
    }

    thread::sleep(Duration::from_secs(1));

    let now: Instant = Instant::now();
    let functions: Vec<fn() -> (String, u8, u8, i16)> = vec![addition, subtraction, multiplication];
    let mut points: u64 = 0;
    while now.elapsed().as_secs() <= timer {
        let mut rng = rand::thread_rng();
        if let Some(rand_fn) = functions.choose(&mut rng){
            let mut ans = rand_fn();
            print!("{}", ans.0);
            io::stdout().flush().unwrap();
            let mut args = String::new();
            io::stdin().read_line(&mut args).unwrap();
            let value: Result<i16, _> = args.trim().parse();
            match value {
                Ok(val) => checker(&val, &ans.3, &mut points),
                Err(_) => (),
            }
        }
    }
    println!("You scored {} points!
    ", points);
    print!("Save your score? (y/n) ");
    io::stdout().flush().unwrap();
    let mut args = String::new();
    io::stdin().read_line(&mut args).unwrap();
    match args.to_lowercase().trim() {
        "y" => save(points),
        "n" => (),
        _ => (),
    }
}

fn addition() -> (String, u8, u8, i16) {
    let x = rand::random::<u8>();
    let y = rand::random::<u8>();
    let ans = x as i16 + y as i16;

    (format!("{} {} {} = ", x.to_string(), String::from("+"), y.to_string()), x, y, ans)
}

fn subtraction() -> (String, u8, u8, i16) {
    let x = rand::random::<u8>();
    let y = rand::thread_rng().gen_range(0..x);
    let ans = x as i16 - y as i16;

    (format!("{} {} {} = ", x.to_string(), String::from("-"), y.to_string()), x, y, ans)
}

fn multiplication() -> (String, u8, u8, i16) {
    let x = rand::random::<u8>() / 100;
    let y = rand::random::<u8>() / 100;
    let ans = x as i16 * y as i16;

    (format!("{} {} {} = ", x.to_string(), String::from("x"), y.to_string()), x, y, ans)
}

fn checker(a: &i16, b: &i16, p: &mut u64) -> () {
    if a == b {
        *p += 1;
    }
}