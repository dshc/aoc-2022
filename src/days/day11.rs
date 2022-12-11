use std::collections::VecDeque;

pub fn solve() {
    let part1_answer = part1();
    println!("part1: {}", part1_answer);
}

fn part1() -> usize {
    let mut monkeys: Vec<Monkey> = include_str!("../../inputs/111real.txt")
        .split("\n\n")
        .map(|line| Monkey::new(line))
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                monkeys[i].inspected_count += 1;
                let item = monkeys[i].items.pop_front().unwrap();

                let new_value = match monkeys[i].operation {
                    Operation::Multiply(x) => item * x,
                    Operation::Add(x) => item + x,
                    Operation::Pow(x) => item.pow(x),
                };

                let new_value = new_value / 3;

                let next_monkey: usize;
                if new_value % monkeys[i].test == 0 {
                    next_monkey = monkeys[i].test_result.on_true;
                } else {
                    next_monkey = monkeys[i].test_result.on_false;
                }

                monkeys[next_monkey].items.push_back(new_value);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));
    monkeys[0].inspected_count * monkeys[1].inspected_count
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    test_result: TestResult,
    inspected_count: usize,
}

impl Monkey {
    pub fn new(input: &str) -> Monkey {
        let lines: Vec<&str> = input.split("\n").map(|line| line.trim()).collect();

        let items = lines[1]
            .split([',', ':'])
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect::<VecDeque<usize>>();

        let operation;
        if lines[2].contains("old * old") {
            operation = Operation::Pow(2);
        } else {
            let mut tokens = lines[2].split(" ").skip(4);
            operation = match tokens.next().unwrap() {
                "*" => Operation::Multiply(tokens.next().unwrap().parse().unwrap()),
                "+" => Operation::Add(tokens.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        }

        let test = lines[3].split(" ").last().unwrap().parse().unwrap();
        let on_true = lines[4].split(" ").last().unwrap().parse().unwrap();
        let on_false = lines[5].split(" ").last().unwrap().parse().unwrap();
        let test_result = TestResult { on_true, on_false };

        Monkey {
            items,
            operation,
            test,
            test_result,
            inspected_count: 0,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Pow(u32),
}

#[derive(Debug)]
struct TestResult {
    on_true: usize,
    on_false: usize,
}
