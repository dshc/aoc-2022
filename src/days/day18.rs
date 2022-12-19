use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../../inputs/181real.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let coords = parse_input(input);
    num_exposed_sides(&coords)
}

fn part2(input: &str) -> usize {
    let coords = parse_input(input);

    // find min/max of x, y, and z coords use those to create a border around the big cube
    let (x_min, x_max, y_min, y_max, z_min, z_max) = get_border(&coords);

    // starting at the border, flood fill as many adjacent cubes as possible while keeping track
    // of them
    let mut steam_coords: HashSet<Coord> = HashSet::new();

    let mut q: VecDeque<Coord> = VecDeque::new();
    q.push_back(Coord {
        x: x_min,
        y: y_min,
        z: z_min,
    });

    while !q.is_empty() {
        let curr_coord = q.pop_front().unwrap();

        if coords.contains(&curr_coord)
            || steam_coords.contains(&curr_coord)
            || curr_coord.x < x_min
            || curr_coord.x > x_max
            || curr_coord.y < y_min
            || curr_coord.y > y_max
            || curr_coord.z < z_min
            || curr_coord.z > z_max
        {
            continue;
        }

        steam_coords.insert(curr_coord);

        for adjacent in adjacent_cubes(&curr_coord) {
            q.push_back(adjacent);
        }
    }

    // count the number of exposed sides just like part1 except this time we check if the cube is
    // adjacent to any of the cubes we found during the flood fill
    num_externally_exposed_sides(&coords, &steam_coords)
}

fn num_externally_exposed_sides(coords: &HashSet<Coord>, steam_coords: &HashSet<Coord>) -> usize {
    let mut exposed_sides = 0;
    for coord in coords {
        for adj_coord in adjacent_cubes(coord) {
            if steam_coords.contains(&adj_coord) {
                exposed_sides += 1;
            }
        }
    }
    exposed_sides
}

fn get_border(coords: &HashSet<Coord>) -> (i32, i32, i32, i32, i32, i32) {
    let x_min = coords.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x - 1;
    let x_max = coords.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 1;
    let y_min = coords.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y - 1;
    let y_max = coords.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 1;
    let z_min = coords.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z - 1;
    let z_max = coords.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z + 1;
    (x_min, x_max, y_min, y_max, z_min, z_max)
}

fn num_exposed_sides(coords: &HashSet<Coord>) -> usize {
    let mut exposed_sides = 0;
    for coord in coords {
        for adj_coord in adjacent_cubes(coord) {
            if !coords.contains(&adj_coord) {
                exposed_sides += 1;
            }
        }
    }
    exposed_sides
}

fn adjacent_cubes(coord: &Coord) -> Vec<Coord> {
    vec![
        Coord {
            x: coord.x + 1,
            y: coord.y,
            z: coord.z,
        },
        Coord {
            x: coord.x - 1,
            y: coord.y,
            z: coord.z,
        },
        Coord {
            x: coord.x,
            y: coord.y + 1,
            z: coord.z,
        },
        Coord {
            x: coord.x,
            y: coord.y - 1,
            z: coord.z,
        },
        Coord {
            x: coord.x,
            y: coord.y,
            z: coord.z + 1,
        },
        Coord {
            x: coord.x,
            y: coord.y,
            z: coord.z - 1,
        },
    ]
}

// TODO - represent coordinates as 3 bitmaps for x,y,z?
// would that be more efficient than a hashset of coords?
fn parse_input(input: &str) -> HashSet<Coord> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (x, y, z) = line
                .trim()
                .split(",")
                .map(|x| match x.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("ERROR PARSING: {}", x);
                        panic!();
                    }
                })
                .collect_tuple()
                .unwrap();
            Coord { x, y, z }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test1() {
        let input = include_str!("../../inputs/181test.txt");
        assert_eq!(part1(input), 64);
    }

    #[test]
    fn part1_example_test2() {
        let input = "1,1,1\n2,1,1";
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn part2_example_test() {
        let input = include_str!("../../inputs/181test.txt");
        assert_eq!(part2(input), 58);
    }
}
