use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../../inputs/091real.txt");
    let part1_result = part1(input);
    println!("part1: {}", part1_result);
    let part2_result = part2(input);
    println!("part2: {}", part2_result);
}

fn part1(input: &str) -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let (mut head, mut tail) = ((0, 0), (0, 0));

    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .for_each(|(dir, count)| {
            for _ in 0..count {
                // move head
                head = move_head_single(head, dir);

                // check distance between head and tail
                let distance = calculate_distance(head, tail);

                // move tail if necessary
                if distance >= 2.0 {
                    tail = move_tail(head, tail);
                }

                // add tail coords to tail_visited
                tail_visited.insert(tail);
            }
        });

    tail_visited.len()
}

fn part2(input: &str) -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut knots = vec![(0, 0); 10];

    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .for_each(|(dir, count)| {
            for _ in 0..count {
                // move first knot
                knots[0] = move_head_single(knots[0], dir);

                for i in 0..knots.len()-1 {
                    let first = knots[i];
                    let second = knots[i+1];
                    // check distance between first and second knots
                    let distance = calculate_distance(first, second);
                    // move second if necessary
                    if distance >= 2.0 {
                        knots[i+1] = move_tail(first, second);
                    }
                }

                // add tail coords to tail_visited
                tail_visited.insert(knots[9]);
            }
        });

    tail_visited.len()
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let should_move_h = head.0 != tail.0;
    let should_move_y = head.1 != tail.1;

    match (should_move_h, should_move_y) {
        (true, true) => {
            let ntx = match head.0 > tail.0 {
                true => tail.0 + 1,
                false => tail.0 - 1,
            };
            let nty = match head.1 > tail.1 {
                true => tail.1 + 1,
                false => tail.1 - 1,
            };
            (ntx, nty)
        }
        (true, false) => {
            if head.0 > tail.0 {
                return (tail.0 + 1, tail.1);
            }
            (tail.0 - 1, tail.1)
        }
        (false, true) => {
            if head.1 > tail.1 {
                return (tail.0, tail.1 + 1);
            }
            (tail.0, tail.1 - 1)
        }
        _ => tail,
    }
}

fn calculate_distance(head: (i32, i32), tail: (i32, i32)) -> f32 {
    ((((tail.0 - head.0).pow(2)) + ((tail.1 - head.1).pow(2))) as f32).sqrt()
}

fn move_head_single(origin: (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "U" => (origin.0, origin.1 + 1),
        "D" => (origin.0, origin.1 - 1),
        "L" => (origin.0 - 1, origin.1),
        "R" => (origin.0 + 1, origin.1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part1_example_calc_distance() {
        assert!(calculate_distance((1, 1), (2, 2)) <= 2.0);
        assert!(calculate_distance((0, 0), (1, 2)) >= 2.0);
    }
}
