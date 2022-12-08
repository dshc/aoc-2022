pub fn solve() {
    let input = include_str!("../../inputs/081real.txt");

    let part1_result = part1(input);
    println!("part1: {}", part1_result);

    let part2_result = part2(input);
    println!("part2: {}", part2_result);
}

fn part1(input: &str) -> usize {
    let lines = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let (height, width) = (lines.len(), lines[0].len());
    let mut visible = vec![false; height * width];

    for x in 0..width {
        visible[x] = true;
        let b = visible.len() - x - 1;
        visible[b] = true;
    }

    for y in 0..height {
        let a = get_index(height, 0, y);
        visible[a] = true;
        let b = get_index(height, width - 1, y);
        visible[b] = true;
    }

    for x in 1..width - 1 {
        // down
        let mut tallest = lines[x][0];
        for y in 1..height - 1 {
            if lines[x][y] > tallest {
                tallest = lines[x][y];
                visible[get_index(height, x, y)] = true;
            }
        }

        // up
        tallest = lines[x][height - 1];
        for y in (1..height - 1).rev() {
            if lines[x][y] > tallest {
                tallest = lines[x][y];
                visible[get_index(height, x, y)] = true;
            }
        }
    }

    for y in 1..height - 1 {
        // right
        let mut tallest = lines[0][y];
        for x in 1..width {
            if lines[x][y] > tallest {
                tallest = lines[x][y];
                visible[get_index(height, x, y)] = true;
            }
        }

        // left
        tallest = lines[width - 1][y];
        for x in (1..width).rev() {
            if lines[x][y] > tallest {
                tallest = lines[x][y];
                visible[get_index(height, x, y)] = true;
            }
        }
    }

    visible.into_iter().filter(|v| *v).count()
}

fn part2(input: &str) -> usize {
    let lines = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let (height, width) = (lines.len(), lines[0].len());
    let mut scores = vec![0; height * width];

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let i = get_index(height, x, y);
            let init_tree_height = lines[x][y];
            let score = dfs(&lines, x - 1, y, "up", init_tree_height)
                * dfs(&lines, x + 1, y, "down", init_tree_height)
                * dfs(&lines, x, y + 1, "right", init_tree_height)
                * dfs(&lines, x, y - 1, "left", init_tree_height);
            scores[i] = score;
        }
    }

    scores.sort_unstable();
    scores.into_iter().last().unwrap()
}

fn dfs(lines: &Vec<Vec<u32>>, x: usize, y: usize, direction: &str, init_tree_height: u32) -> usize {
    if x == 0
        || x == lines[0].len() - 1
        || y == 0
        || y == lines.len() - 1
        || lines[x][y] >= init_tree_height
    {
        return 1;
    }

    1 + match direction {
        "up" => dfs(lines, x - 1, y, "up", init_tree_height),
        "down" => dfs(lines, x + 1, y, "down", init_tree_height),
        "right" => dfs(lines, x, y + 1, "right", init_tree_height),
        _ => dfs(lines, x, y - 1, "left", init_tree_height),
    }
}

fn get_index(height: usize, x: usize, y: usize) -> usize {
    x + (y * height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        let input = "30373\n\
                     25512\n\
                     65332\n\
                     33549\n\
                     35390";

        assert_eq!(part1(input), 21);
    }

    #[test]
    fn part2_example_test() {
        let input = "30373\n\
                     25512\n\
                     65332\n\
                     33549\n\
                     35390";

        assert_eq!(part2(input), 8);
    }
}
