use std::collections::HashSet;

pub fn solve() {
    let map = include_str!("../../inputs/221realmap.txt");
    let dirs = include_str!("../../inputs/221realdir.txt");
    println!("part1: {}", part1(map, dirs));
}

fn part1(map: &str, dirs: &str) -> usize {
    let instructions = parse_instructions(dirs);
    // println!("{:#?}", instructions);
    let (rows, walls) = parse_map(map);
    // println!("{:#?}", rows);
    // println!("{:#?}", walls);

    let mut state = (rows[0].min, 0, Direction::Right);

    for instruction in instructions {
        match instruction {
            Instruction::MoveStraight(distance) => {
                let new_coord =
                    move_to_new_coord(&state.2, distance, (state.0, state.1), &rows, &walls);
                state = (new_coord.0, new_coord.1, state.2);
            }
            Instruction::TurnRight => {
                let new_dir = match state.2 {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                state = (state.0, state.1, new_dir);
            }
            Instruction::TurnLeft => {
                let new_dir = match state.2 {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                state = (state.0, state.1, new_dir);
            }
        }
    }

    let res = ((state.1 + 1) * 1000) + ((state.0 + 1) * 4);

    match state.2 {
        Direction::Up => res + 3,
        Direction::Down => res + 1,
        Direction::Left => res + 2,
        Direction::Right => res,
    }
}

fn move_to_new_coord(
    dir: &Direction,
    distance: usize,
    curr_coord: (usize, usize),
    rows: &Vec<Row>,
    walls: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut coord = curr_coord;
    match dir {
        Direction::Up => {
            for _ in 0..distance {
                coord = move_vertical_single(coord, rows, walls, true);
            }
        }
        Direction::Down => {
            for _ in 0..distance {
                coord = move_vertical_single(coord, rows, walls, false);
            }
        }
        Direction::Right => {
            for _ in 0..distance {
                coord = move_horizontal_single(coord, &rows[coord.1], walls, false);
            }
        }
        Direction::Left => {
            for _ in 0..distance {
                coord = move_horizontal_single(coord, &rows[coord.1], walls, true);
            }
        }
    }

    coord
}

fn move_horizontal_single(
    init: (usize, usize),
    row: &Row,
    walls: &HashSet<(usize, usize)>,
    left: bool,
) -> (usize, usize) {
    let mut next_x = init.0;

    if left && next_x == row.min {
        next_x = row.max;
    } else if !left && next_x == row.max {
        next_x = row.min;
    } else {
        if left {
            next_x -= 1;
        } else {
            next_x += 1;
        }
    }

    if walls.contains(&(next_x, init.1)) {
        init
    } else {
        (next_x, init.1)
    }
}

fn move_vertical_single(
    init: (usize, usize),
    rows: &Vec<Row>,
    walls: &HashSet<(usize, usize)>,
    up: bool,
) -> (usize, usize) {
    let mut next_y = init.1;

    let acceptable_rows = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.min <= init.0 && row.max >= init.0)
        .map(|(y, _)| y)
        .collect::<Vec<usize>>();

    if up && next_y == 0 {
        next_y = acceptable_rows.into_iter().max().unwrap();
    } else if !up && next_y == rows.len() - 1 {
        next_y = acceptable_rows.into_iter().min().unwrap();
    } else {
        next_y = if up { next_y - 1 } else { next_y + 1 };
        if !acceptable_rows.contains(&next_y) {
            if up {
                next_y = acceptable_rows.into_iter().max().unwrap();
            } else {
                next_y = acceptable_rows.into_iter().min().unwrap();
            }
        }
    }

    if walls.contains(&(init.0, next_y)) {
        init
    } else {
        (init.0, next_y)
    }
}

fn parse_instructions(dirs: &str) -> Vec<Instruction> {
    dirs.trim()
        .split_inclusive(['R', 'L'])
        .flat_map(|line| {
            let mut res: Vec<Instruction> = Vec::new();

            if line.ends_with('R') {
                res.push(Instruction::MoveStraight(
                    line.trim_matches('R').parse().unwrap(),
                ));
                res.push(Instruction::TurnRight);
            } else if line.ends_with('L') {
                res.push(Instruction::MoveStraight(
                    line.trim_matches('L').parse().unwrap(),
                ));
                res.push(Instruction::TurnLeft);
            } else {
                res.push(Instruction::MoveStraight(line.parse().unwrap()));
            }

            res
        })
        .collect()
}

fn parse_map(map: &str) -> (Vec<Row>, HashSet<(usize, usize)>) {
    let mut walls = HashSet::new();
    let mut rows = Vec::new();

    map.trim_end()
        .split('\n')
        .enumerate()
        .for_each(|(y, line)| {
            let min = line.find(|c| c == '.' || c == '#').unwrap();
            let max = line.rfind(|c| c == '.' || c == '#').unwrap();
            rows.push(Row { min, max });

            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    walls.insert((x, y));
                });
        });

    (rows, walls)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    MoveStraight(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Debug)]
struct Row {
    min: usize,
    max: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let map = include_str!("../../inputs/221testmap.txt");
        let dirs = include_str!("../../inputs/221testdir.txt");
        assert_eq!(part1(map, dirs), 6032);
    }
}
