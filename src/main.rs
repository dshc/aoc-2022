mod days;

use std::{env, time::Instant};

use crate::days::{day01, day02};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of arguments provided.");
    }

    let day_choice: u8 = args[1].parse().unwrap();
    println!("Selected day {}", day_choice);

    let now = Instant::now();
    match day_choice {
        1 => day01::solve(),
        2 => day02::solve(),
        _ => unimplemented!(),
    }
    println!("Timer (ms): {:#?}", now.elapsed());
}
