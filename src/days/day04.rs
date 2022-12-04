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
        .filter(|x| (x[0] >= x[2] && x[1] <= x[3]) || (x[0] <= x[2] && x[1] >= x[3]))
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
        .filter(|x| (x[0] <= x[3]) && (x[1] >= x[2]))
        .count()
}
