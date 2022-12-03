use std::{
    collections::HashSet,
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
        let one = into_hashset(&x[0]);
        let two = into_hashset(&x[1]);
        for c in x[2].chars() {
            if one.contains(&c) && two.contains(&c) {
                result += char_to_priority(c);
                break;
            }
        }
    }

    result
}

fn into_hashset(s: &str) -> HashSet<char> {
    let mut found = HashSet::new();
    for c in s.chars() {
        found.insert(c);
    }
    found
}

fn part1(lines: &Vec<String>) -> usize {
    let mut result = 0;
    for line in lines {
        let mid = line.len() / 2;
        let mut seen = HashSet::new();

        let chars: Vec<char> = line.chars().collect();
        for i in 0..line.len() {
            let c = chars[i];
            if i < mid {
                seen.insert(c);
            } else {
                if seen.contains(&c) {
                    result += char_to_priority(c);
                    break;
                }
            }
        }
    }
    result
}

fn char_to_priority(c: char) -> usize {
    let t = c as usize + 1;
    return match t {
        65..=91 => 26 + t - 'A' as usize,
        _ => t - 'a' as usize,
    };
}
