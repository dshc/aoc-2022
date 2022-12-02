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

fn calc_score_part2(opponent: &str, player: &str) -> u32 {
    let mut score = 0;

    match player {
        "X" => {
            match opponent {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => unimplemented!(),
            }
        }
        "Y" => {
            match opponent {
                "A" => score += 4,
                "B" => score += 5,
                "C" => score += 6,
                _ => unimplemented!(),
            }
        }
        "Z" => {
            match opponent {
                "A" => score += 8,
                "B" => score += 9,
                "C" => score += 7,
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }

    score
}

fn calc_score_part1(opponent: &str, player: &str) -> u32 {
    let mut score = 0;

    match player {
        "X" => {
            score += 1;
            match opponent {
                "A" => score += 3,
                "B" => score += 0,
                "C" => score += 6,
                _ => unimplemented!(),
            }
        }
        "Y" => {
            score += 2;
            match opponent {
                "A" => score += 6,
                "B" => score += 3,
                "C" => score += 0,
                _ => unimplemented!(),
            }
        }
        "Z" => {
            score += 3;
            match opponent {
                "A" => score += 0,
                "B" => score += 6,
                "C" => score += 3,
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }

    score
}
