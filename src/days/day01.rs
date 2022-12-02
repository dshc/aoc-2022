use std::{
    fs::File,
    io::{self, BufRead}, collections::BinaryHeap,
};

const _INPUT_PATH_REAL: &str = "./inputs/011real.txt";
const _INPUT_PATH_TEST: &str = "./inputs/011test.txt";

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
    Some(get_sorted_calories(_INPUT_PATH_REAL).pop().unwrap())
}

fn part2() -> Option<u32> {
    let mut sorted = get_sorted_calories(_INPUT_PATH_REAL);
    let mut result = 0;
    for _ in 0..3 {
        result += sorted.pop().unwrap();
    }
    Some(result)
}

fn get_sorted_calories(path: &str) -> BinaryHeap<u32> {
    let mut cals_by_elf = BinaryHeap::new();
    get_lines(path).iter().fold(0, |acc, x| match x {
        None => {
            cals_by_elf.push(acc);
            0
        }
        Some(cals) => acc + cals,
    });
    cals_by_elf
}

fn get_lines(path: &str) -> Vec<Option<u32>> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|x| {
            let temp_str = x.unwrap();
            let i = temp_str.parse::<u32>();
            match i {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        })
        .collect::<Vec<Option<u32>>>()
}
