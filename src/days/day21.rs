use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../../inputs/211real.txt");
    println!("part1: {}", part1(input));
    let input = include_str!("../../inputs/212real.txt");
    println!(
        "part2: {}",
        part2(input, 3300000000000, 3400000000000, true)
    );
}

fn part1(input: &str) -> f64 {
    let monkeys = parse_monkeys(input);

    let mut monkey_yells: HashMap<&str, f64> = monkeys
        .iter()
        .filter(|m| if let Op::None = m.op { true } else { false })
        .map(|m| (m.name, m.yell))
        .collect();

    while !monkey_yells.contains_key("root") {
        for monkey in &monkeys {
            if monkey_yells.contains_key(monkey.name) {
                continue;
            }

            if !monkey_yells.contains_key(monkey.dep_a) || !monkey_yells.contains_key(monkey.dep_b)
            {
                continue;
            }

            let yell = match monkey.op {
                Op::None => unreachable!(),
                Op::Add => monkey_yells[monkey.dep_a] + monkey_yells[monkey.dep_b],
                Op::Sub => monkey_yells[monkey.dep_a] - monkey_yells[monkey.dep_b],
                Op::Mul => monkey_yells[monkey.dep_a] * monkey_yells[monkey.dep_b],
                Op::Div => monkey_yells[monkey.dep_a] / monkey_yells[monkey.dep_b],
                Op::Eq => unreachable!(),
            };

            monkey_yells.insert(monkey.name, yell);
        }
    }

    monkey_yells["root"]
}

// seems like "humn" yell affects dep_a. Let's find a possible range for humn and then do a binary search
// within that given range. from manually checking, it seems like a decent range is (3300000000000, 3400000000000).
fn part2(input: &str, init_low: i64, init_high: i64, inverse: bool) -> i64 {
    let monkeys = parse_monkeys(input);

    let mut low: i64 = init_low;
    let mut high: i64 = init_high;

    while high >= low {
        let mut monkey_yells: HashMap<&str, f64> = monkeys
            .iter()
            .filter(|m| if let Op::None = m.op { true } else { false })
            .map(|m| (m.name, m.yell))
            .collect();

        let mid = low + (high - low) / 2;
        monkey_yells.insert("humn", mid as f64);

        while !monkey_yells.contains_key("root") {
            for monkey in &monkeys {
                if monkey_yells.contains_key(monkey.name) {
                    continue;
                }

                if !monkey_yells.contains_key(monkey.dep_a)
                    || !monkey_yells.contains_key(monkey.dep_b)
                {
                    continue;
                }

                let yell = match monkey.op {
                    Op::None => unreachable!(),
                    Op::Add => monkey_yells[monkey.dep_a] + monkey_yells[monkey.dep_b],
                    Op::Sub => monkey_yells[monkey.dep_a] - monkey_yells[monkey.dep_b],
                    Op::Mul => monkey_yells[monkey.dep_a] * monkey_yells[monkey.dep_b],
                    Op::Div => monkey_yells[monkey.dep_a] / monkey_yells[monkey.dep_b],
                    Op::Eq => {
                        let a = monkey_yells[monkey.dep_a];
                        let b = monkey_yells[monkey.dep_b];
                        if a == b {
                            0.0
                        } else if a > b {
                            1.0
                        } else {
                            -1.0
                        }
                    }
                };

                monkey_yells.insert(monkey.name, yell);
            }
        }

        if monkey_yells["root"] == 0.0 {
            return mid;
        }

        if inverse {
            if monkey_yells["root"] > 0.0 {
                // a is higher than b which means the human needs to yell a higher number
                low = mid + 1;
            } else {
                // a is lower than b which means the human needs to yell a lower number
                high = mid - 1;
            }
        } else {
            if monkey_yells["root"] > 0.0 {
                // a is higher than b which means the human needs to yell a lower number
                high = mid - 1;
            } else {
                // a is lower than b which means the human needs to yell a higher number
                low = mid + 1;
            }
        }
    }

    0
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .map(|words| {
            let name = words[0].trim_matches(':');
            if words.len() == 2 {
                Monkey {
                    name,
                    yell: words[1].parse().unwrap(),
                    dep_a: "",
                    dep_b: "",
                    op: Op::None,
                }
            } else {
                let op = match words[2] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    "=" => Op::Eq,
                    _ => unreachable!(),
                };
                Monkey {
                    name,
                    yell: 0.0,
                    dep_a: words[1],
                    dep_b: words[3],
                    op,
                }
            }
        })
        .collect()
}

#[derive(Debug)]
struct Monkey<'a> {
    name: &'a str,
    yell: f64,
    dep_a: &'a str,
    dep_b: &'a str,
    op: Op,
}

#[derive(Debug)]
enum Op {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../../inputs/211test.txt");
        assert_eq!(part1(input), 152.0);
    }

    #[test]
    fn example_2() {
        let input = include_str!("../../inputs/212test.txt");
        assert_eq!(part2(input, 1, 500, false), 301);
    }
}
