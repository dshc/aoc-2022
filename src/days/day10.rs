pub fn solve() {
    let input_str = include_str!("../../inputs/101real.txt");
    println!("part1: {}", part1(input_str));
}

fn part1(input_str: &str) -> i32 {
    let a = [20, 60, 100, 140, 180, 220];

    input_str
        .lines()
        .flat_map(|line| {
            if line.starts_with("n") {
                return vec![line];
            }
            return vec!["noop", line];
        })
        .fold(vec![0, 1], |mut registers, action| {
            if action.starts_with("addx") {
                let add_val: i32 = action.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
                let new_xreg = add_val + registers.last().unwrap();
                registers.push(new_xreg);
            } else {
                registers.push(*registers.last().unwrap());
            }
            registers
        })
        .into_iter()
        .enumerate()
        .fold(0, |acc, (cycle, register)| {
            if a.contains(&cycle) {
                let add = cycle as i32 * register;
                return acc + add;
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input_str = include_str!("../../inputs/101test.txt");
        assert_eq!(part1(input_str), 13140);
    }
}
