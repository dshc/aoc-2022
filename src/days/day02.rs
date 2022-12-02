use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let mut score_part1 = 0;
    let mut score_part2 = 0;
    let file = File::open("./inputs/021real.txt");
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        let l = line.unwrap();
        let mut moves = l.split(' ');
        let opp = moves.next().unwrap();
        let player = moves.next().unwrap();
        score_part1 += calc_score_part1(opp, player);
        score_part2 += calc_score_part2(opp, player);
    }
    println!("part1: {}", score_part1);
    println!("part2: {}", score_part2);
}

fn exec(opponent: &str, a: u32, b: u32, c: u32) -> u32 {
    match opponent {
        "A" => return a,
        "B" => return b,
        "C" => return c,
        _ => unimplemented!(),
    }
}

fn calc_score_part2(opponent: &str, player: &str) -> u32 {
    match player {
        "X" => return exec(opponent, 3, 1, 2),
        "Y" => return exec(opponent, 4, 5, 6),
        "Z" => return exec(opponent, 8, 9, 7),
        _ => unimplemented!(),
    }
}

fn calc_score_part1(opponent: &str, player: &str) -> u32 {
    match player {
        "X" => return exec(opponent, 4, 1, 7),
        "Y" => return exec(opponent, 8, 5, 2),
        "Z" => return exec(opponent, 3, 9, 6),
        _ => unimplemented!(),
    }
}
