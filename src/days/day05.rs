use std::collections::VecDeque;

pub fn solve() {
    let x = include_str!("../../inputs/051real.txt")
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let init = &x[0];
    let instructions = &x[1];

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    init.iter().for_each(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|x| {
                x.iter()
                    .filter(|c| c.is_ascii_uppercase())
                    .collect::<Vec<&char>>()
            })
            .enumerate()
            .for_each(|(i, x)| {
                if stacks.len() < i + 1 {
                    stacks.push(VecDeque::new())
                }

                if x.len() > 0 {
                    stacks[i].push_front(*x[0]);
                }
            })
    });

    instructions.iter().for_each(|instruction| {
        let x = instruction
            .split(" ")
            .filter_map(|temp| temp.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        if x.len() == 0 {
            return;
        }

        let count = x[0];
        let from = x[1];
        let to = x[2];

        for _ in 0..count {
            let t = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(t);
        }
    });

    let mut result = String::new();
    for i in 0..stacks.len() {
        result += &stacks[i].pop_back().unwrap().to_string();
    }

    println!("{}", result);
}
