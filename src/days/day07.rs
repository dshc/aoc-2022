use std::collections::HashMap;

pub fn solve() {
    let lines = include_str!("../../inputs/071real.txt")
        .split("$")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .skip(1);

    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut curr_dir_stack: Vec<String> = Vec::new();
    curr_dir_stack.push(String::from(""));

    for line in lines {
        match line.starts_with("cd") {
            true => {
                let selected_dir = line.split(" ").skip(1).next().unwrap();
                if selected_dir == ".." {
                    curr_dir_stack.pop();
                } else {
                    let prev_dir = curr_dir_stack.last().unwrap();
                    let x = format!("{}/{}", prev_dir, selected_dir);
                    curr_dir_stack.push(x);
                }
            }
            false => {
                let files_total_size: usize = line
                    .split("\n")
                    .flat_map(|l| l.split(" "))
                    .filter_map(|w| w.parse::<usize>().ok())
                    .sum();

                for d in &curr_dir_stack {
                    let v = sizes.entry(d.to_string()).or_insert(0);
                    *v += files_total_size;
                }
            }
        }
    }

    let result: usize = sizes.iter().map(|s| *s.1).filter(|s| *s < 100000).sum();
    println!("part1: {}", result);

    let total_space: usize = 70000000;
    let goal_unused: usize = 30000000;
    let curr_used: &usize = sizes.get("").unwrap();
    let to_free: usize = *curr_used - (total_space - goal_unused);
    let mut candidates: Vec<usize> = sizes.iter().map(|s| *s.1).filter(|s| *s >= to_free).collect();
    candidates.sort();
    println!("part2: {}", candidates.first().unwrap());
}
