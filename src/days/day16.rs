use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../../inputs/161real.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let valves = parse_input(input);
    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
    dfs(&valves, 0, 30, 0, 0 << valves.len(), &mut cache)
}

fn part2(input: &str) -> usize {
    let _valves = parse_input(input);
    0
}

fn dfs(
    all_valves: &HashMap<usize, Valve>,
    curr_valve: usize,
    time_remaining: usize,
    pressure_released: usize,
    open_valves: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if time_remaining <= 0 {
        return pressure_released;
    }

    let curr_pressure = pressure_released + calc_pressure_added(open_valves, all_valves);

    let cache_key = (time_remaining, curr_valve, curr_pressure);

    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    let mut max = 0;
    if all_valves[&curr_valve].flow_rate > 0 && open_valves & (1 << curr_valve) == 0 {
        // Open the valve
        max = dfs(
            all_valves,
            curr_valve,
            time_remaining - 1,
            curr_pressure,
            open_valves | (1 << curr_valve),
            cache,
        );
    }

    for next_valve in all_valves[&curr_valve].tunnels.iter() {
        // traverse tunnels
        max = std::cmp::max(
            max,
            dfs(
                all_valves,
                *next_valve,
                time_remaining - 1,
                curr_pressure,
                open_valves,
                cache,
            ),
        );
    }

    cache.insert(cache_key, max);

    max
}

fn calc_pressure_added(open_valves: usize, all_valves: &HashMap<usize, Valve>) -> usize {
    all_valves
        .iter()
        .filter(|valve| (1 << *valve.0) & open_valves == (1 << *valve.0))
        .map(|valve| valve.1.flow_rate)
        .sum()
}

fn parse_input(input: &str) -> HashMap<usize, Valve> {
    let mut x: Vec<(String, usize, Vec<String>)> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(";");
            let name = parts.next().unwrap().to_string();
            let flow_rate = parts.next().unwrap().parse::<usize>().unwrap();
            let tunnels = parts
                .flat_map(|s| s.split(","))
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (name.to_owned(), flow_rate, tunnels)
        })
        .collect();

    x.sort_by(|a, b| a.0.cmp(&b.0));

    let ids: HashMap<String, usize> = x
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, y)| (y.0, i))
        .collect();

    x.into_iter()
        .map(|y| {
            let tunnels = y.2.iter().map(|t| ids[t]).collect();
            (
                ids[&y.0],
                Valve {
                    flow_rate: y.1,
                    tunnels,
                },
            )
        })
        .collect()
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = include_str!("../../inputs/161test.txt");
        assert_eq!(part1(input), 1651);
    }

    #[test]
    fn part2_example_test() {
        let input = include_str!("../../inputs/161test.txt");
        assert_eq!(part2(input), 1707);
    }
}
