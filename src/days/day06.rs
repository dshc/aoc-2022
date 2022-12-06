use itertools::Itertools;

pub fn solve() {
    let s = include_str!("../../inputs/061real.txt");
    println!("part1: {}", part1(s));
    println!("part2: {}", part2(s));
}

pub fn part1(s: &str) -> usize {
    find_start(s, 4)
}

pub fn part2(s: &str) -> usize {
    find_start(s, 14)
}

pub fn find_start(s: &str, window_size: usize) -> usize {
    let chars = s.chars().collect::<Vec<char>>();
    chars
        .windows(window_size)
        .enumerate()
        .filter(|(_, window)| window.into_iter().all_unique())
        .map(|(i, _)| i+window_size)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
