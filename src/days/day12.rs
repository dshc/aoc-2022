use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let input_str = include_str!("../../inputs/121real.txt");
    println!("part1: {}", part1(input_str));
    println!("part2: {}", part2(input_str));
}

pub fn part1(input: &str) -> usize {
    let (grid, start, end) = init_grid(input);

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
    q.push_front((start, 0));
    visited.insert(start);

    let len_x = grid[0].len() as i32;
    let len_y = grid.len() as i32;

    while q.len() > 0 {
        let (curr_coord, curr_depth) = q.pop_back().unwrap();

        if curr_coord == end {
            return curr_depth;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = curr_coord.x as i32 + dx;
            let new_y = curr_coord.y as i32 + dy;

            if new_y < 0
                || new_x < 0
                || new_y >= len_y
                || new_x >= len_x
                || grid[curr_coord.y][curr_coord.x] + 1 < grid[new_y as usize][new_x as usize]
            {
                continue;
            }

            let new_coord = Coord {
                x: new_x as usize,
                y: new_y as usize,
            };

            if !visited.contains(&new_coord) {
                q.push_front((new_coord, curr_depth + 1));
                visited.insert(new_coord);
            }
        }
    }

    0
}

pub fn part2(input: &str) -> usize {
    let (grid, _, end) = init_grid(input);

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
    q.push_front((end, 0));
    visited.insert(end);

    let len_x = grid[0].len() as i32;
    let len_y = grid.len() as i32;

    while q.len() > 0 {
        let (curr_coord, curr_depth) = q.pop_back().unwrap();

        if grid[curr_coord.y][curr_coord.x] == 'a' as usize {
            return curr_depth;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = curr_coord.x as i32 + dx;
            let new_y = curr_coord.y as i32 + dy;

            if new_y < 0
                || new_x < 0
                || new_y >= len_y
                || new_x >= len_x
                || grid[curr_coord.y][curr_coord.x] > grid[new_y as usize][new_x as usize] + 1
            {
                continue;
            }

            let new_coord = Coord {
                x: new_x as usize,
                y: new_y as usize,
            };

            if !visited.contains(&new_coord) {
                q.push_front((new_coord, curr_depth + 1));
                visited.insert(new_coord);
            }
        }
    }

    0
}

fn init_grid(input: &str) -> (Vec<Vec<usize>>, Coord, Coord) {
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    grid.iter().enumerate().for_each(|(y, vc)| {
        vc.iter().enumerate().for_each(|(x, c)| match *c {
            'S' => start = Coord { x, y },
            'E' => end = Coord { x, y },
            _ => (),
        })
    });

    let grid: Vec<Vec<usize>> = grid
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| match c {
                    'S' => 'a' as usize,
                    'E' => 'z' as usize,
                    _ => c as usize,
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    (grid, start, end)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_eq_test() {
        let a = Coord { x: 1, y: 3 };
        let b = Coord { x: 1, y: 3 };
        assert_eq!(a, b);
    }

    #[test]
    fn coord_ne_test() {
        let a = Coord { x: 2, y: 3 };
        let b = Coord { x: 1, y: 3 };
        assert_ne!(a, b);
    }

    #[test]
    fn part1_example_test() {
        let input = "Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi";

        assert_eq!(part1(input), 31);
    }

    #[test]
    fn part2_example_test() {
        let input = "Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi";

        assert_eq!(part2(input), 29);
    }
}
