use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../../inputs/171real.txt");
    println!("part1: {}", part1(input, 2022));
}

fn part1(input: &str, num_rocks: usize) -> usize {
    let mut gas_direction_iter = parse_input(input);
    let mut rock_shape_iter = get_rock_shape_iter();
    let mut playing_field = vec![HashSet::from([0]); 7];

    let mut max_height = 0;

    for _ in 0..num_rocks {
        let mut rock_bottom_height = max_height + 4;
        let rock = rock_shape_iter.next().unwrap();
        let mut pieces = rock.get_pieces();

        translate(Direction::Right, 2, &mut pieces);
        translate(Direction::Up, rock_bottom_height, &mut pieces);

        loop {
            if let Some(dir) = gas_direction_iter.next() {
                if can_move(&dir, &pieces, &playing_field) {
                    translate(dir, 1, &mut pieces);
                }
            }

            if can_move(&Direction::Down, &pieces, &playing_field) {
                translate(Direction::Down, 1, &mut pieces);
                rock_bottom_height -= 1;
            } else {
                break;
            }
        }

        update_playing_field(&mut playing_field, &pieces);

        max_height = std::cmp::max(max_height, rock_bottom_height + rock.height - 1);
    }

    max_height
}

/* fn get_field_max_height(playing_field: &Vec<HashSet<usize>>) -> usize {
    let mut max = 0;
    for set in playing_field {
        max = std::cmp::max(max, *set.iter().max().unwrap());
    }
    max
} */

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

fn parse_input(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .cycle()
}

fn get_rock_shape_iter() -> impl Iterator<Item = RockShape<'static>> {
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
    .cycle()
}

fn translate(direction: Direction, num_moves: usize, pieces: &mut Vec<(usize, usize)>) {
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
}
