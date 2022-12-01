mod days;

use std::env;

use crate::days::day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of arguments provided.");
    }

    let day_choice: u8 = args[1].parse().unwrap();
    println!("Selected day {}", day_choice);

    match day_choice {
        1 => day01::solve(),
        _ => unimplemented!(),
    }
}
