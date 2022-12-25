use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("../../inputs/231real.txt");
    println!("part1: {}", part1(input, 10));
    println!("part2: {}", part2(input));
}

fn part2(input: &str) -> usize {
    let mut elf_coords = parse_input(input);

    let mut round_num = 0;
    loop {
        let mut new_coords: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for orig_elf_coord in &elf_coords {
            let (changed, new_elf_coord) = calc_new_coord(&orig_elf_coord, &elf_coords, round_num % 4);
            if changed {
                if !new_coords.contains_key(&new_elf_coord) {
                    new_coords.insert(new_elf_coord, Vec::new());
                }

                new_coords
                    .get_mut(&new_elf_coord)
                    .unwrap()
                    .push(*orig_elf_coord);
            }
        }

        if new_coords.len() == 0 {
            return round_num + 1;
        }

        for (new, old_coords) in &new_coords {
            if old_coords.len() == 1 {
                elf_coords.remove(old_coords.first().unwrap());
                elf_coords.insert(*new);
            }
        }

        round_num += 1;
    }
}

fn part1(input: &str, num_rounds: usize) -> usize {
    let mut elf_coords = parse_input(input);

    for round in 0..num_rounds {
        // for each elf, determine if it can be moved.
        // if it can, then store the new coord in a hashmap of <new coord, old coord>
        // if there is already something in the hashmap with the new coord as a key, remove
        // that key entirely from the hashmap. Once we've done that for all coords,
        // go through each key in the new hashmap and update the elf_coords hashmap by
        // removing the old coord and inserting the new coord

        let mut new_coords: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for orig_elf_coord in &elf_coords {
            let (changed, new_elf_coord) = calc_new_coord(&orig_elf_coord, &elf_coords, round % 4);
            if changed {
                if !new_coords.contains_key(&new_elf_coord) {
                    new_coords.insert(new_elf_coord, Vec::new());
                }

                new_coords
                    .get_mut(&new_elf_coord)
                    .unwrap()
                    .push(*orig_elf_coord);
            }
        }

        if new_coords.len() == 0 {
            break;
        }

        for (new, old_coords) in &new_coords {
            if old_coords.len() == 1 {
                elf_coords.remove(old_coords.first().unwrap());
                elf_coords.insert(*new);
            }
        }
    }

    // get min & max of x & y to calculate x length and y length for the area.
    // subtract the number of elves from the area and that's your answer!
    let min_x = elf_coords.iter().map(|&coord| coord.0).min().unwrap();
    let max_x = elf_coords.iter().map(|&coord| coord.0).max().unwrap();
    let min_y = elf_coords.iter().map(|&coord| coord.1).min().unwrap();
    let max_y = elf_coords.iter().map(|&coord| coord.1).max().unwrap();

    let x = (max_x - min_x + 1) as usize;
    let y = (max_y - min_y + 1) as usize;

    (x * y) - elf_coords.len()
}

const NEIGHBOR_DISTANCES: &[(i32, i32)] = &[
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
];

const NORTH_DISTANCES: &[(i32, i32)] = &[(-1, -1), (0, -1), (1, -1)];
const SOUTH_DISTANCES: &[(i32, i32)] = &[(-1, 1), (0, 1), (1, 1)];
const WEST_DISTANCES: &[(i32, i32)] = &[(-1, 1), (-1, 0), (-1, -1)];
const EAST_DISTANCES: &[(i32, i32)] = &[(1, 1), (1, 0), (1, -1)];

const DISTANCES: &[((i32, i32), &[(i32, i32)])] = &[
    ((0, -1), NORTH_DISTANCES),
    ((0, 1), SOUTH_DISTANCES),
    ((-1, 0), WEST_DISTANCES),
    ((1, 0), EAST_DISTANCES),
];

fn calc_new_coord(
    curr: &(i32, i32),
    elf_coords: &HashSet<(i32, i32)>,
    order_to_check_offset: usize,
) -> (bool, (i32, i32)) {
    // check if any other elf is close by first. if not, then stay where you are
    if !NEIGHBOR_DISTANCES
        .iter()
        .any(|d| elf_coords.contains(&(curr.0 + d.0, curr.1 + d.1)))
    {
        return (false, *curr);
    }

    if let Some((dist, _)) = DISTANCES
        .iter()
        .cycle()
        .skip(order_to_check_offset)
        .take(4)
        .find(|(_, distances)| {
            !distances
                .iter()
                .any(|d| elf_coords.contains(&(curr.0 + d.0, curr.1 + d.1)))
        })
    {
        (true, (curr.0 + dist.0, curr.1 + dist.1))
    } else {
        (false, *curr)
    }
}

// parse the input into a hashset containing all of the coordinates of the elves
fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example2() {
        let input = include_str!("../../inputs/231test2.txt");
        assert_eq!(part1(input, 10), 25);
    }

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/231test.txt");
        assert_eq!(part1(input, 10), 110);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../inputs/231test.txt");
        assert_eq!(part2(input), 20);
    }
}
