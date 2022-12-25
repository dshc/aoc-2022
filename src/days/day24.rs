use std::{collections::HashSet, hash::Hash};

use bitflags::bitflags;

pub fn solve() {
    let input = include_str!("../../inputs/241real.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part2(input: &str) -> usize {
    let field = parse_input(input);
    let mut result = 0;

    let (to_add, field) = exec((1, 0), (field[0].len() - 2, field.len() - 1), &field);
    result += to_add;

    let (to_add, field) = exec((field[0].len() - 2, field.len() - 1), (1, 0), &field);
    result += to_add;

    let (to_add, _) = exec((1, 0), (field[0].len() - 2, field.len() - 1), &field);
    result += to_add;

    result
}

fn exec(
    start: (usize, usize),
    goal: (usize, usize),
    init_field: &Vec<Vec<Blizzard>>,
) -> (usize, Vec<Vec<Blizzard>>) {
    let mut field = init_field.clone();

    // begin to end
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(start);

    for time in 0.. {
        field = update_field(&field);

        let mut next_positions: HashSet<(usize, usize)> = HashSet::new();
        for p in &positions {
            for pp in open_positions(p.0, p.1, &field) {
                if pp.0 == goal.0 && pp.1 == goal.1 {
                    return (time + 1, field);
                }
                next_positions.insert(pp);
            }
        }

        positions = next_positions;
    }

    (0, field)
}

fn part1(input: &str) -> usize {
    let mut field = parse_input(input);

    let goal = (field[0].len() - 2, field.len() - 1);

    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert((1, 0));

    for time in 0.. {
        field = update_field(&field);

        let mut next_positions: HashSet<(usize, usize)> = HashSet::new();
        for p in &positions {
            if p.0 == goal.0 && p.1 == goal.1 {
                return time;
            }

            for pp in open_positions(p.0, p.1, &field) {
                next_positions.insert(pp);
            }
        }

        positions = next_positions;
    }

    0
}

fn open_positions(x: usize, y: usize, next_field: &Vec<Vec<Blizzard>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if next_field[y][x].is_empty() {
        res.push((x, y));
    }

    // check left
    if x > 0 && next_field[y][x - 1].is_empty() {
        res.push((x - 1, y));
    }

    // check right
    if x < next_field[0].len() - 1 && next_field[y][x + 1].is_empty() {
        res.push((x + 1, y));
    }

    // check up
    if y > 0 && next_field[y - 1][x].is_empty() {
        res.push((x, y - 1));
    }

    // check down
    if y < next_field.len() - 1 && next_field[y + 1][x].is_empty() {
        res.push((x, y + 1));
    }

    res
}

fn update_field(field: &Vec<Vec<Blizzard>>) -> Vec<Vec<Blizzard>> {
    let mut new_field = vec![vec![Blizzard::empty(); field[0].len()]; field.len()];

    for y in 0..field.len() {
        for x in 0..field[0].len() {
            if field[y][x].is_empty() {
                continue;
            }

            if field[y][x] == Blizzard::WALL {
                new_field[y][x].insert(Blizzard::WALL);
                continue;
            }

            if field[y][x].contains(Blizzard::UP) {
                let mut next_y = y - 1;
                if field[next_y][x] == Blizzard::WALL {
                    next_y = field.len() - 2;
                }

                new_field[next_y][x].insert(Blizzard::UP);
            }

            if field[y][x].contains(Blizzard::DOWN) {
                let mut next_y = y + 1;
                if field[next_y][x] == Blizzard::WALL {
                    next_y = 1;
                }

                new_field[next_y][x].insert(Blizzard::DOWN);
            }

            if field[y][x].contains(Blizzard::LEFT) {
                let mut next_x = x - 1;
                if field[y][next_x] == Blizzard::WALL {
                    next_x = field[0].len() - 2;
                }

                new_field[y][next_x].insert(Blizzard::LEFT);
            }

            if field[y][x].contains(Blizzard::RIGHT) {
                let mut next_x = x + 1;
                if field[y][next_x] == Blizzard::WALL {
                    next_x = 1;
                }

                new_field[y][next_x].insert(Blizzard::RIGHT);
            }
        }
    }

    new_field
}

fn parse_input(input: &str) -> Vec<Vec<Blizzard>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Blizzard::empty(),
                    '#' => Blizzard::WALL,
                    '^' => Blizzard::UP,
                    'v' => Blizzard::DOWN,
                    '<' => Blizzard::LEFT,
                    '>' => Blizzard::RIGHT,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

bitflags! {
    struct Blizzard: u8 {
        const WALL = 0b00001;
        const UP = 0b00010;
        const DOWN = 0b00100;
        const LEFT = 0b01000;
        const RIGHT = 0b10000;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/241test.txt");
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/241test.txt");
        assert_eq!(part2(input), 54);
    }
}
