use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("../../inputs/151real.txt");
    println!("part1: {}", part1(input, 2_000_000));
}

fn part1(input: &str, check_row: isize) -> usize {
    let readings = parse_input(input);
    let mut grid: HashMap<Coord, Space> = HashMap::new();

    readings.iter().for_each(|reading| {
        grid.insert(reading.closest_beacon, Space::Beacon);
        grid.insert(reading.sensor, Space::Sensor);
    });

    // get max possible distance based on input
    let max_distance = readings
        .iter()
        .map(|reading| reading.distance())
        .max()
        .unwrap();

    // get min max of x coord of all sensors
    let max_x = readings
        .iter()
        .map(|reading| reading.sensor.x)
        .max()
        .unwrap()
        + max_distance;
    let min_x = readings
        .iter()
        .map(|reading| reading.sensor.x)
        .min()
        .unwrap()
        - max_distance;

    let mut count = 0;

    for x in min_x..=max_x {
        let curr_coord = Coord { x, y: check_row };

        // if grid contains the curr_coord, its either a sensor or beacon already
        if grid.contains_key(&curr_coord) {
            continue;
        }

        // check if curr_coord is close enough to any sensor
        for reading in &readings {
            if calc_distance(&curr_coord, &reading.sensor) <= reading.distance() {
                count += 1;
                break;
            }
        }
    }

    count
}

fn _update_range(grid: &mut HashMap<Coord, Space>, sensor: &Coord, max_distance: isize) {
    let dirs = &[(0, 1), (1, 0), (-1, 0), (0, -1)];

    let mut s: Vec<Coord> = Vec::new();
    grid.insert(*sensor, Space::Sensor);
    s.push(*sensor);

    let mut seen: HashSet<Coord> = HashSet::new();
    seen.insert(*sensor);

    while s.len() > 0 {
        let curr_coord = s.pop().unwrap();

        for dir in dirs {
            let next_coord = Coord {
                x: curr_coord.x + dir.0,
                y: curr_coord.y + dir.1,
            };

            if seen.contains(&next_coord) {
                continue;
            }

            if calc_distance(&next_coord, sensor) <= max_distance {
                if !grid.contains_key(&next_coord) {
                    grid.insert(next_coord, Space::Empty);
                }
                s.push(next_coord);
                seen.insert(next_coord);
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<DeviceReading> {
    input
        .trim()
        .split("\n")
        .map(|line| DeviceReading::new(line))
        .collect()
}

fn calc_distance(a: &Coord, b: &Coord) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug)]
enum Space {
    Empty,
    Sensor,
    Beacon,
}

#[derive(Debug)]
struct DeviceReading {
    sensor: Coord,
    closest_beacon: Coord,
}

impl DeviceReading {
    pub fn new(line: &str) -> Self {
        let t: Vec<&str> = line.split(":").collect();
        let sensor = Coord::new(t[0]);
        let closest_beacon = Coord::new(t[1]);
        Self {
            sensor,
            closest_beacon,
        }
    }
    pub fn distance(&self) -> isize {
        calc_distance(&self.sensor, &self.closest_beacon)
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(input: &str) -> Self {
        let i: Vec<isize> = input.split(",").map(|s| s.parse().unwrap()).collect();
        Self { x: i[0], y: i[1] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = include_str!("../../inputs/151test.txt");
        assert_eq!(part1(input, 10), 26);
    }
}
