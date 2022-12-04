pub fn solve() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> usize {
    include_str!("../../inputs/041real.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .flat_map(|x| x.split("-"))
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(4)
        .filter(|x| {
            let first = x[0]..=x[1];
            let second = x[2]..=x[3];
            (first.contains(&x[2]) && first.contains(&x[3]))
                || (second.contains(&x[0]) && second.contains(&x[1]))
        })
        .count()
}

fn part2() -> usize {
    include_str!("../../inputs/041real.txt")
        .lines()
        .flat_map(|line| line.split(","))
        .flat_map(|x| x.split("-"))
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(4)
        .filter(|x| {
            let first = x[0]..=x[1];
            let second = x[2]..=x[3];
            (first.contains(&x[2]) || first.contains(&x[3]))
                || (second.contains(&x[0]) || second.contains(&x[1]))
        })
        .count()
}
