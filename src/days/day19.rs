use rayon::prelude::*;
use std::collections::HashMap;

use regex::Regex;

pub fn solve() {
    let input: &str = include_str!("../../inputs/191real.txt");
    println!("part1: {}", part1(input));
    let input: &str = include_str!("../../inputs/192real.txt");
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .par_iter()
        .map(|blueprint| {
            let ore_costs = [
                blueprint.ore_robot_cost.ore,
                blueprint.clay_robot_cost.ore,
                blueprint.obsidian_robot_cost.ore,
                blueprint.geode_robot_cost.ore,
            ];

            let res = simulate(
                24,
                (0, 0, 0, 0),
                (1, 0, 0, 0),
                (
                    *ore_costs.iter().max().unwrap(),
                    blueprint.obsidian_robot_cost.clay,
                    blueprint.geode_robot_cost.obsidian,
                ),
                blueprint,
                &mut HashMap::new(),
            );

            return res * blueprint.id;
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .par_iter()
        .map(|blueprint| {
            let ore_costs = [
                blueprint.ore_robot_cost.ore,
                blueprint.clay_robot_cost.ore,
                blueprint.obsidian_robot_cost.ore,
                blueprint.geode_robot_cost.ore,
            ];

            let res = simulate(
                32,
                (0, 0, 0, 0),
                (1, 0, 0, 0),
                (
                    *ore_costs.iter().max().unwrap(),
                    blueprint.obsidian_robot_cost.clay,
                    blueprint.geode_robot_cost.obsidian,
                ),
                blueprint,
                &mut HashMap::new(),
            );

            return res;
        })
        .product()
}

fn simulate(
    time: u32,
    resources: (u32, u32, u32, u32), //(ore, clay, obsidian, geode)
    robots: (u32, u32, u32, u32),
    max_costs: (u32, u32, u32),
    blueprint: &Blueprint,
    cache: &mut HashMap<(u32, u32, u32, u32, u32, u32, u32, u32), (u32, u32)>,
) -> u32 {
    if time == 0 {
        return resources.3;
    }

    let key = &(
        resources.0,
        resources.1,
        resources.2,
        resources.3,
        robots.0,
        robots.1,
        robots.2,
        robots.3,
    );

    if cache.contains_key(key) {
        let (t, g) = cache.get(key).unwrap();
        if time <= *t {
            return *g;
        }
    }

    //determine which robots are affordable
    let can_build_ore_robot = can_afford(resources, &blueprint.ore_robot_cost);
    let can_build_clay_robot = can_afford(resources, &blueprint.clay_robot_cost);
    let can_build_obsidian_robot = can_afford(resources, &blueprint.obsidian_robot_cost);
    let can_build_geode_robot = can_afford(resources, &blueprint.geode_robot_cost);

    //collect resources
    let resources = (
        resources.0 + robots.0,
        resources.1 + robots.1,
        resources.2 + robots.2,
        resources.3 + robots.3,
    );

    let mut max = 0;

    if can_build_geode_robot {
        max = std::cmp::max(
            max,
            simulate(
                time - 1,
                build_robot(resources, &blueprint.geode_robot_cost),
                (robots.0, robots.1, robots.2, robots.3 + 1),
                max_costs,
                blueprint,
                cache,
            ),
        );
    } else {
        if can_build_obsidian_robot && robots.2 < max_costs.2 {
            max = std::cmp::max(
                max,
                simulate(
                    time - 1,
                    build_robot(resources, &blueprint.obsidian_robot_cost),
                    (robots.0, robots.1, robots.2 + 1, robots.3),
                    max_costs,
                    blueprint,
                    cache,
                ),
            );
        }

        if can_build_clay_robot && robots.1 < max_costs.1 {
            max = std::cmp::max(
                max,
                simulate(
                    time - 1,
                    build_robot(resources, &blueprint.clay_robot_cost),
                    (robots.0, robots.1 + 1, robots.2, robots.3),
                    max_costs,
                    blueprint,
                    cache,
                ),
            );
        }

        if can_build_ore_robot && robots.0 < max_costs.0 {
            max = std::cmp::max(
                max,
                simulate(
                    time - 1,
                    build_robot(resources, &blueprint.ore_robot_cost),
                    (robots.0 + 1, robots.1, robots.2, robots.3),
                    max_costs,
                    blueprint,
                    cache,
                ),
            );
        }

        // if you can build any robot you want, then it makes no sense to not build anything
        if !can_build_ore_robot
            || !can_build_clay_robot
            || !can_build_obsidian_robot
            || !can_build_geode_robot
        {
            max = std::cmp::max(
                max,
                simulate(time - 1, resources, robots, max_costs, blueprint, cache),
            );
        }
    }

    cache.insert(*key, (time, max));

    max
}

fn can_afford(resources: (u32, u32, u32, u32), cost: &MaterialCost) -> bool {
    resources.0 >= cost.ore && resources.1 >= cost.clay && resources.2 >= cost.obsidian
}

fn build_robot(resources: (u32, u32, u32, u32), cost: &MaterialCost) -> (u32, u32, u32, u32) {
    (
        resources.0 - cost.ore,
        resources.1 - cost.clay,
        resources.2 - cost.obsidian,
        resources.3,
    )
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let mut caps = re.find_iter(line);
            Blueprint {
                id: caps.next().unwrap().as_str().parse().unwrap(),
                ore_robot_cost: MaterialCost {
                    ore: caps.next().unwrap().as_str().parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                clay_robot_cost: {
                    MaterialCost {
                        ore: caps.next().unwrap().as_str().parse().unwrap(),
                        clay: 0,
                        obsidian: 0,
                    }
                },
                obsidian_robot_cost: {
                    MaterialCost {
                        ore: caps.next().unwrap().as_str().parse().unwrap(),
                        clay: caps.next().unwrap().as_str().parse().unwrap(),
                        obsidian: 0,
                    }
                },
                geode_robot_cost: {
                    MaterialCost {
                        ore: caps.next().unwrap().as_str().parse().unwrap(),
                        clay: 0,
                        obsidian: caps.next().unwrap().as_str().parse().unwrap(),
                    }
                },
            }
        })
        .collect()
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: MaterialCost,
    clay_robot_cost: MaterialCost,
    obsidian_robot_cost: MaterialCost,
    geode_robot_cost: MaterialCost,
}

#[derive(Debug)]
struct MaterialCost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input: &str = include_str!("../../inputs/191test.txt");
        assert_eq!(part1(input), 33);
    }

    #[test]
    fn part1_example_test2() {
        let input: &str = include_str!("../../inputs/191test2.txt");
        assert_eq!(part1(input), 9);
    }

    #[test]
    fn part1_example_test3() {
        let input: &str = include_str!("../../inputs/191test3.txt");
        assert_eq!(part1(input), 189);
    }

    // #[test]
    fn _part2_example_test() {
        let input: &str = include_str!("../../inputs/191test.txt");
        assert_eq!(part2(input), 56 * 62);
    }

    // #[test]
    fn _part2_example_test2() {
        let input: &str = include_str!("../../inputs/191test2.txt");
        assert_eq!(part2(input), 56);
    }
}
