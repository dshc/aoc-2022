#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, BufRead},
};

const INPUT_PATH_REAL: &str = "./inputs/011real.txt";
const INPUT_PATH_TEST: &str = "./inputs/011test.txt";

pub fn solve() {
    let part1 = part1();
    match part1 {
        Some(x) => println!("Part 1: {}", x),
        None => println!("Part 1: Not Implemented"),
    }

    let part2 = part2();
    match part2 {
        Some(x) => println!("Part 2: {}", x),
        None => println!("Part 2: Not Implemented"),
    }
}

fn part1() -> Option<u32> {
    let mut max: u32 = 0;

    get_line_iter(INPUT_PATH_REAL).fold(0, |acc, x| match x.unwrap().trim() {
        "" => {
            if acc > max {
                max = acc
            };
            0
        }
        cals => acc + cals.parse::<u32>().unwrap(),
    });

    return Some(max);
}

fn part2() -> Option<u32> {
    let mut cals_by_elf = Vec::new();

    get_line_iter(INPUT_PATH_REAL).fold(0, |acc, x| match x.unwrap().trim() {
        "" => {
            cals_by_elf.push(acc);
            0
        }
        cals => acc + cals.parse::<u32>().unwrap(),
    });

    cals_by_elf.sort();

    Some(cals_by_elf.iter().rev().take(3).sum())
}

fn get_line_iter(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    return reader.lines();
}
