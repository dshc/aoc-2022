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
    let mut curr_elf: u32 = 0;

    get_line_iter(INPUT_PATH_REAL).for_each(|x| match x.unwrap().trim() {
        "" => {
            if curr_elf > max {
                max = curr_elf
            }
            curr_elf = 0;
        }
        cals => curr_elf += cals.parse::<u32>().unwrap(),
    });

    if curr_elf != 0 && curr_elf > max {
        max = curr_elf;
    }

    return Some(max);
}

fn part2() -> Option<u32> {
    let mut cals_by_elf = Vec::new();
    let mut curr_elf = 0;

    get_line_iter(INPUT_PATH_REAL).for_each(|x| match x.unwrap().trim() {
        "" => {
            cals_by_elf.push(curr_elf);
            curr_elf = 0;
        }
        cals => curr_elf += cals.parse::<u32>().unwrap(),
    });

    if curr_elf != 0 {
        cals_by_elf.push(curr_elf);
    }

    cals_by_elf.sort();

    let mut sum = 0;
    for _ in 0..3 {
        sum += cals_by_elf.pop().unwrap();
    }

    return Some(sum);
}

fn get_line_iter(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    return reader.lines();
}
