use std::collections::VecDeque;

pub fn solve() {
    let x = include_str!("../../inputs/051real.txt")
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let init = &x[0];
    let instructions = &x[1];

    println!("part1: {}", part1(init, instructions));
    println!("part2: {}", part2(init, instructions));
}

fn part2(init: &Vec<&str>, instructions: &Vec<&str>) -> String {
    let mut stacks = initialize_stacks(init);

    parse_instructions(instructions).iter().for_each(|x| {
        let count = x[0];
        let from = x[1];
        let to = x[2];

        let mut temp_vec = VecDeque::new();

        for _ in 0..count {
            let t = stacks[from - 1].pop_back().unwrap();
            temp_vec.push_back(t);
        }
        for _ in 0..temp_vec.len() {
            let t = temp_vec.pop_back().unwrap();
            stacks[to - 1].push_back(t);
        }
    });

    calc_result(&mut stacks)
}

fn part1(init: &Vec<&str>, instructions: &Vec<&str>) -> String {
    let mut stacks = initialize_stacks(init);

    parse_instructions(instructions).iter().for_each(|x| {
        let count = x[0];
        let from = x[1];
        let to = x[2];

        for _ in 0..count {
            let t = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(t);
        }
    });

    calc_result(&mut stacks)
}

fn parse_instructions(inst: &Vec<&str>) -> Vec<Vec<usize>> {
    inst.iter()
        .map(|x| {
            x.split(" ")
                .filter_map(|temp| temp.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .filter(|x| x.len() > 0)
        .collect()
}

fn calc_result(stacks: &mut Vec<VecDeque<char>>) -> String {
    let mut result = String::new();
    for i in 0..stacks.len() {
        result += &stacks[i].pop_back().unwrap().to_string();
    }
    result
}

fn initialize_stacks(init: &Vec<&str>) -> Vec<VecDeque<char>> {
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

    stacks
}
