use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("../../inputs/171real.txt");
    println!("2022 iterations: {}", part1(input, 2022));
    // println!("1 trillion iterations: {}", part1(input, 1000000000000));
}

fn part1(input: &str, num_rocks: usize) -> usize {
    let mut gas_direction_iter = parse_input(input);
    let mut rock_shape_iter = get_rock_shape_iter();
    let mut playing_field = vec![HashSet::from([0]); 7];
    let mut max_height = 0;

    // cache key ==> (rock_index, gas_index, [distance of each column's max height to the total max
    // height of the playing field])
    let mut cache: HashMap<
        (
            usize, // rock_index
            usize, // gas_index
            usize, // x = 0
            usize, // x = 1
            usize, // x = 2
            usize, // x = 3
            usize, // x = 4
            usize, // x = 5
            usize, // x = 6
        ),
        (usize, usize), // max height, rock number
    > = HashMap::new();

    let mut skip_ahead = false;
    let mut rock_number = 0;
    while rock_number < num_rocks {
        let mut rock_bottom_height = max_height + 4;
        let (rock_index, rock) = rock_shape_iter.next().unwrap();
        let mut pieces = rock.get_pieces();

        translate(&Direction::Right, 2, &mut pieces);
        translate(&Direction::Up, rock_bottom_height, &mut pieces);

        let mut init_gas_index_set = false;
        let mut init_gas_index = 0;

        loop {
            let (gas_index, dir) = gas_direction_iter.next().unwrap();
            if !init_gas_index_set && !skip_ahead {
                init_gas_index = gas_index;
                init_gas_index_set = true;

                let k = &get_cache_key(rock_index, init_gas_index, max_height, &playing_field);

                if cache.contains_key(k) {
                    let (prev_height, prev_rock_number) = cache.get(k).unwrap();
                    let rocks_to_skip = rock_number - prev_rock_number;
                    let height_to_add = max_height - prev_height;
                    let num_cycles_to_skip_ahead = (num_rocks - rock_number) / rocks_to_skip;
                    rock_number += num_cycles_to_skip_ahead * rocks_to_skip;
                    max_height += num_cycles_to_skip_ahead * height_to_add;
                    println!("new rock number: {}", rock_number);
                    println!("new max_height: {}", max_height);
                    skip_ahead = true;

                    // update pieces of current rock
                    translate(&Direction::Up, max_height, &mut pieces);

                    // update all playing field pieces
                    let new_playing_field = playing_field
                        .iter()
                        .map(|set| {
                            set.iter()
                                .map(|x| *x + max_height)
                                .collect::<HashSet<usize>>()
                        })
                        .collect::<Vec<HashSet<usize>>>();
                    playing_field = new_playing_field;
                }
            }

            if can_move(&dir, &pieces, &playing_field) {
                translate(&dir, 1, &mut pieces);
            }

            if can_move(&Direction::Down, &pieces, &playing_field) {
                translate(&Direction::Down, 1, &mut pieces);
                rock_bottom_height -= 1;
            } else {
                break;
            }
        }

        update_playing_field(&mut playing_field, &pieces);

        max_height = std::cmp::max(max_height, rock_bottom_height + rock.height - 1);

        if !skip_ahead {
            cache.insert(
                get_cache_key(rock_index, init_gas_index, max_height, &playing_field),
                (max_height, rock_number),
            );
        }

        rock_number += 1;
    }

    max_height
}

fn get_cache_key(
    rock_index: usize,
    gas_index: usize,
    total_max_height: usize,
    playing_field: &Vec<HashSet<usize>>,
) -> (
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) {
    let x = playing_field
        .iter()
        .map(|set| {
            let temp = *set.iter().max().unwrap();
            total_max_height - temp
        })
        .collect::<Vec<usize>>();
    (
        rock_index, gas_index, x[0], x[1], x[2], x[3], x[4], x[5], x[6],
    )
}

fn update_playing_field(playing_field: &mut Vec<HashSet<usize>>, pieces: &Vec<(usize, usize)>) {
    for (x, y) in pieces {
        playing_field[*x].insert(*y);
    }
}

fn can_move(
    dir: &Direction,
    pieces: &Vec<(usize, usize)>,
    playing_field: &Vec<HashSet<usize>>,
) -> bool {
    match dir {
        Direction::Down => {
            for (x, y) in pieces {
                if *y == 0 || playing_field[*x].contains(&(*y - 1)) {
                    return false;
                }
            }
            return true;
        }
        Direction::Up => {
            for (x, y) in pieces {
                if playing_field[*x].contains(&(*y + 1)) {
                    return false;
                }
            }
            return true;
        }
        Direction::Left => {
            for (x, y) in pieces {
                if *x == 0 || playing_field[*x - 1].contains(y) {
                    return false;
                }
            }
            return true;
        }
        Direction::Right => {
            for (x, y) in pieces {
                if *x == playing_field.len() - 1 || playing_field[*x + 1].contains(y) {
                    return false;
                }
            }
            return true;
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, Direction)> + '_ {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .enumerate()
        .cycle()
}

fn get_rock_shape_iter() -> impl Iterator<Item = (usize, RockShape<'static>)> {
    vec![
        // -
        RockShape {
            pieces: &[(0, 0), (1, 0), (2, 0), (3, 0)],
            height: 1,
        },
        // +
        RockShape {
            pieces: &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            height: 3,
        },
        // backwards L
        RockShape {
            pieces: &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            height: 3,
        },
        // l
        RockShape {
            pieces: &[(0, 0), (0, 1), (0, 2), (0, 3)],
            height: 4,
        },
        // square
        RockShape {
            pieces: &[(0, 0), (1, 0), (0, 1), (1, 1)],
            height: 2,
        },
    ]
    .into_iter()
    .enumerate()
    .cycle()
}

fn translate(direction: &Direction, num_moves: usize, pieces: &mut Vec<(usize, usize)>) {
    match direction {
        Direction::Up => pieces.iter_mut().for_each(|(_, y)| *y += num_moves),
        Direction::Down => pieces.iter_mut().for_each(|(_, y)| *y -= num_moves),
        Direction::Left => pieces.iter_mut().for_each(|(x, _)| *x -= num_moves),
        Direction::Right => pieces.iter_mut().for_each(|(x, _)| *x += num_moves),
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct RockShape<'a> {
    pieces: &'a [(usize, usize)],
    height: usize,
}

impl RockShape<'_> {
    fn get_pieces(&self) -> Vec<(usize, usize)> {
        self.pieces.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(part1(input, 2022), 3068);
    }

    // #[test]
    // fn part2_example_test() {
    //     let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    //     assert_eq!(part1(input, 1000000000000), 1514285714288);
    // }
}
