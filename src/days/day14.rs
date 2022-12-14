use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../../inputs/141real.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let all_rocks = get_all_rocks(input)
        .into_iter()
        .map(|c| Coord {
            x: c.x - 400,
            y: c.y,
        })
        .collect_vec();

    let mut grid = vec![vec![false; 200]; 200];
    all_rocks.iter().for_each(|coord| {
        grid[coord.x][coord.y] = true;
    });

    for y in 0..200 {
        for x in 0..200 {
            if y == 0 && x == 100 {
                print!("+");
            }
            else if grid[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    0
}

fn part2(input: &str) -> usize {
    0
}

fn get_all_rocks(input: &str) -> Vec<Coord> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" -> ").map(|p| Coord::new(p)).collect_vec())
        .flat_map(|coords| {
            coords
                .windows(2)
                .map(|c| all_points_between(c[0], c[1]))
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn all_points_between(a: Coord, b: Coord) -> Vec<Coord> {
    let mut result = Vec::new();

    if a.x == b.x {
        let x = a.x;
        let mut range = a.y..=b.y;
        if a.y > b.y {
            range = b.y..=a.y;
        }
        for y in range {
            result.push(Coord { x, y });
        }
    } else {
        let y = a.y;
        let mut range = a.x..=b.x;
        if a.x > b.x {
            range = b.x..=a.x;
        }
        for x in range {
            result.push(Coord { x, y });
        }
    }

    result
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(point: &str) -> Self {
        let mut s = point.split(",");
        Self {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part1(input), 24);
    }
}
