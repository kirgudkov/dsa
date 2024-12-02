fn solve(input: &str, is_valid: fn(&[i32], [i32; 2]) -> bool) -> i32 {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    input
        .iter()
        .fold(0, |acc, row| if is_valid(row, [1, 3]) { acc + 1 } else { acc })
}

fn part_1(vec: &[i32], range: [i32; 2]) -> bool {
    is_valid(vec, &range)
}

fn part_2(vec: &[i32], range: [i32; 2]) -> bool {
    is_valid(vec, &range) || vec.iter().enumerate().any(|(i, _)| {
        let mut temp = vec.to_vec();
        temp.remove(i);
        is_valid(&temp, &range)
    })
}

fn is_valid(vec: &[i32], range: &[i32; 2]) -> bool {
    let is_increasing = vec.is_sorted_by(|a, b| a < b);
    let is_decreasing = vec.is_sorted_by(|a, b| a > b);
    // Checks if each pair of elements in the vector is well spaced (i.e. the difference between them is within the range)
    let is_well_spaced = vec
        .windows(2)
        .all(|pair| (pair[0] - pair[1]).abs() >= range[0] && (pair[0] - pair[1]).abs() <= range[1]);

    is_well_spaced && (is_increasing || is_decreasing)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input, part_1), 2);
        dbg!(solve(&read_to_string("./src/aoc/inputs/day_2").unwrap(), part_1));
    }

    #[test]
    fn test_part_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
1 3 2 1 0
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input, part_2), 5);
        dbg!(solve(&read_to_string("./src/aoc/inputs/day_2").unwrap(), part_2));
    }
}
