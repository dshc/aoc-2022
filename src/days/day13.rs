use std::cmp::Ordering;

pub fn solve() {
    let input = include_str!("../../inputs/131real.txt");
    println!("part1: {}", part1(input));
    let input = include_str!("../../inputs/132real.txt");
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| compare(pair))
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| convert(s))
        .collect::<Vec<Packet>>();
    let six = &Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let two = &Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    packets.sort_by(compare_packets);
    packets.iter().enumerate().fold(1, |acc, (i, packet)| {
        if compare_packets(packet, two) == Ordering::Equal
            || compare_packets(packet, six) == Ordering::Equal
        {
            acc * (i+1)
        } else {
            acc
        }
    })
}

fn compare(pair: &str) -> bool {
    let pair = pair.split("\n").collect::<Vec<&str>>();
    let l = convert(pair[0]);
    let r = convert(pair[1]);
    let o = compare_packets(&l, &r);
    o == Ordering::Less || o == Ordering::Equal
}

fn compare_packets(l: &Packet, r: &Packet) -> Ordering {
    match (l, r) {
        (Packet::List(left), Packet::List(right)) => {
            let mut a = left.iter();
            let mut b = right.iter();

            loop {
                match (a.next(), b.next()) {
                    (Some(l1), Some(r1)) => {
                        let res = compare_packets(l1, r1);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (None, None) => return Ordering::Equal,
                }
            }
        }
        (Packet::Number(_), Packet::List(right)) => {
            if right.len() == 0 {
                return Ordering::Greater;
            }

            match compare_packets(l, &right[0]) {
                Ordering::Equal => Ordering::Less,
                res => res,
            }
        }
        (Packet::List(left), Packet::Number(_)) => {
            if left.len() == 0 {
                return Ordering::Less;
            }

            match compare_packets(&left[0], r) {
                Ordering::Equal => Ordering::Greater,
                res => res,
            }
        }
        (Packet::Number(left), Packet::Number(right)) => left.cmp(right),
    }
}

fn convert(packet: &str) -> Packet {
    let mut result_packet: Packet = Packet::List(Vec::new());

    let mut curr_depth = 0;
    let mut curr_string = String::new();
    packet.replace("10", "a").chars().for_each(|c| match c {
        '[' => {
            curr_depth += 1;
            if curr_depth > 1 {
                curr_string.push(c);
            }
        }
        ']' => {
            if curr_depth > 1 {
                curr_string.push(c);
            }
            curr_depth -= 1;
            if curr_depth == 1 {
                if let Packet::List(v) = &mut result_packet {
                    v.push(convert(&curr_string));
                    curr_string = String::new();
                }
            }
        }
        ',' => {
            if curr_depth > 1 {
                curr_string.push(c);
            }
        }
        _ => {
            if curr_depth > 1 {
                curr_string.push(c);
            } else if let Packet::List(v) = &mut result_packet {
                v.push(Packet::Number(match c {
                    'a' => 10,
                    _ => c.to_digit(10).unwrap(),
                }));
            }
        }
    });

    result_packet
}

#[derive(Debug)]
enum Packet {
    Number(u32),
    List(Vec<Self>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare1() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]";
        assert!(compare(input));
    }

    #[test]
    fn compare2() {
        let input = "[[1],[2,3,4]]\n[[1],4]";
        assert!(compare(input));
    }

    #[test]
    fn compare4() {
        let input = "[[4,4],4,4]\n[[4,4],4,4,4]";
        assert!(compare(input));
    }

    #[test]
    fn compare5() {
        let input = "[7,7,7,7]\n[7,7,7]";
        assert!(!compare(input));
    }

    #[test]
    fn compare6() {
        let input = "[]\n[3]";
        assert!(compare(input));
    }

    #[test]
    fn compare7() {
        let input = "[[[]]]\n[[]]";
        assert!(!compare(input));
    }

    #[test]
    fn compare8() {
        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert!(!compare(input));
    }

    #[test]
    fn compare9() {
        let input = "[1,2,3]\n[1,[2,3]]";
        assert!(compare(input));
    }

    #[test]
    fn test_equality() {
        let input = "[[6]]";
        let input = convert(input);
        assert_eq!(
            compare_packets(
                &input,
                &Packet::List(vec![Packet::List(vec![Packet::Number(6)])])
            ),
            Ordering::Equal
        );
    }

    #[test]
    fn part1_example_test() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part2_example_test() {
        let input = "[[2]]
[[6]]

[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(part2(input), 140);
    }
}
