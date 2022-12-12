mod days;

use crate::days::*;
use std::{env, time::Instant};

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
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => day09::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        _ => unimplemented!(),
    }
    println!("Timer (ms): {:#?}", now.elapsed());
}
