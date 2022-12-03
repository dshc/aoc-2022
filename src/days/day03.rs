use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("./inputs/031real.txt");
    let reader = BufReader::new(file.unwrap());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}

fn part2(lines: &Vec<String>) -> usize {
    let mut result = 0;
    let mut iter = lines.chunks(3);
    while let Some(x) = iter.next() {
        for c in x[2].chars() {
            if x[0].contains(c) && x[1].contains(c) {
                result += char_to_priority(c);
                break;
            }
        }
    }
    result
}

fn part1(lines: &Vec<String>) -> usize {
    let mut result = 0;
    lines.iter().for_each(|l| {
        let s = l.split_at(l.len() / 2);
        for c in s.1.chars() {
            if s.0.contains(c) {
                result += char_to_priority(c);
                break;
            }
        }
    });
    result
}

fn char_to_priority(c: char) -> usize {
    let t = c as usize + 1;
    return match t {
        65..=91 => 26 + t - 'A' as usize,
        _ => t - 'a' as usize,
    };
}
