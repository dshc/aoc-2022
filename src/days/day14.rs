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

    let mut grid = vec![vec![Material::Air; 200]; 200];
    all_rocks.iter().for_each(|coord| {
        grid[coord.x][coord.y] = Material::Rock;
    });

    // start dropping sand
    let mut counter = 0;
    loop {
        let mut curr_coord = Coord { x: 100, y: 0 };

        loop {
            if curr_coord.y >= 199 {
                // _print_grid_part1(&grid);
                return counter;
            }

            // check directly below
            if let Material::Air = grid[curr_coord.x][curr_coord.y + 1] {
                curr_coord.y += 1;
            }
            // check down to left
            else if let Material::Air = grid[curr_coord.x - 1][curr_coord.y + 1] {
                curr_coord.x -= 1;
                curr_coord.y += 1;
            }
            // check down to right
            else if let Material::Air = grid[curr_coord.x + 1][curr_coord.y + 1] {
                curr_coord.x += 1;
                curr_coord.y += 1;
            } else {
                grid[curr_coord.x][curr_coord.y] = Material::Sand;
                counter += 1;
                break;
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let all_rocks = get_all_rocks(input)
        .into_iter()
        .map(|c| Coord {
            x: c.x,
            y: c.y,
        })
        .collect_vec();

    let floor = all_rocks
            .iter()
            .max_by(|a, b| a.y.cmp(&b.y))
            .unwrap()
            .y + 2;

    let mut grid = vec![vec![Material::Air; 200]; 1000];
    all_rocks.iter().for_each(|coord| {
        grid[coord.x][coord.y] = Material::Rock;
    });
    for x in 0..1000 {
        grid[x][floor] = Material::Rock;
    }

    // start dropping sand
    let mut counter = 0;
    loop {
        let mut curr_coord = Coord { x: 500, y: 0 };

        loop {
            if let Material::Sand = grid[500][0] {
                // _print_grid_part2(&grid);
                return counter;
            }

            // check directly below
            if let Material::Air = grid[curr_coord.x][curr_coord.y + 1] {
                curr_coord.y += 1;
            }
            // check down to left
            else if let Material::Air = grid[curr_coord.x - 1][curr_coord.y + 1] {
                curr_coord.x -= 1;
                curr_coord.y += 1;
            }
            // check down to right
            else if let Material::Air = grid[curr_coord.x + 1][curr_coord.y + 1] {
                curr_coord.x += 1;
                curr_coord.y += 1;
            } else {
                grid[curr_coord.x][curr_coord.y] = Material::Sand;
                counter += 1;
                break;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}

fn _print_grid_part2(grid: &Vec<Vec<Material>>) {
    for y in 0..200 {
        for x in 0..1000 {
            if y == 0 && x == 500 {
                print!("+");
            } else if let Material::Rock = grid[x][y] {
                print!("#");
            } else if let Material::Sand = grid[x][y] {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn _print_grid_part1(grid: &Vec<Vec<Material>>) {
    for y in 0..200 {
        for x in 0..200 {
            if y == 0 && x == 100 {
                print!("+");
            } else if let Material::Rock = grid[x][y] {
                print!("#");
            } else if let Material::Sand = grid[x][y] {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
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

    #[test]
    fn part2_example_test() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part2(input), 93);
    }
}
