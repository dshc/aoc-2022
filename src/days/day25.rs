pub fn solve() {
    let input = include_str!("../../inputs/251real.txt");
    println!("part1: {}", part1(input));
    // println!("part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let dec = input.trim().lines().map(|line| parse_snafu(line)).sum();
    dec_to_snafu(dec)
}

fn dec_to_snafu(dec: i128) -> String {
    let mut res = String::new();

    let mut d = dec;

    while d > 0 {
        match d % 5 {
            0 => res = "0".to_owned() + &res,
            1 => res = "1".to_owned() + &res,
            2 => res = "2".to_owned() + &res,
            3 => {
                res = "=".to_owned() + &res;
                d += 5;
            }
            4 => {
                res = "-".to_owned() + &res;
                d += 5;
            }
            _ => unreachable!(),
        }

        d /= 5;
    }

    res
}

fn parse_snafu(snafu: &str) -> i128 {
    snafu
        .trim_end()
        .chars()
        .map(|c| match c {
            '-' => -1,
            '=' => -2,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        })
        .rev()
        .enumerate()
        .map(|(i, val)| (5 as i128).pow(i as u32) * val)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../inputs/251test.txt");
        assert_eq!(part1(input), "2=-1=0");
    }

    #[test]
    fn part1_example2() {
        let input = "1=11-2";
        assert_eq!(part1(input), "1=11-2");
    }
}
