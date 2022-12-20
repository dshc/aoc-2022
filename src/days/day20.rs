pub fn solve() {
    let input = include_str!("../../inputs/201real.txt");
    println!("part1: {}", exec(input, 1, 1));
    println!("part2: {}", exec(input, 811_589_153, 10));
}

fn exec(input: &str, decryption_key: i64, number_of_mixes: u32) -> i64 {
    let values: Vec<i64> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect();

    let mut indices: Vec<usize> = values.iter().enumerate().map(|(i, _)| i).collect();

    for _ in 0..number_of_mixes {
        for original_index in 0..values.len() {
            let current_index = indices.iter().position(|&x| x == original_index).unwrap();
            let positions_to_move = values[original_index];
            let new_index =
                (current_index as i64 + positions_to_move).rem_euclid(values.len() as i64 - 1);
            let temp = indices.remove(current_index);
            indices.insert(new_index as usize, temp);
        }
    }

    let original_zero_index = values.iter().position(|&x| x == 0).unwrap();
    let new_zero_index = indices
        .iter()
        .position(|&x| x == original_zero_index)
        .unwrap();

    let a = values[indices[(new_zero_index + 1000) % values.len()]];
    let b = values[indices[(new_zero_index + 2000) % values.len()]];
    let c = values[indices[(new_zero_index + 3000) % values.len()]];

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input = include_str!("../../inputs/201test.txt");
        assert_eq!(exec(input, 1, 1), 3);
        assert_eq!(exec(input, 811589153, 10), 1623178306);
    }
}
