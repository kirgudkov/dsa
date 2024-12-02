use std::collections::HashMap;

fn part_1(input: &str) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(input: &str) -> i32 {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut pair = line.split_whitespace();
            (pair.next().unwrap().parse().unwrap(), pair.next().unwrap().parse().unwrap())
        })
        .collect();

    let counts = pairs
        .iter()
        .fold(HashMap::new(), |mut acc, &(_, right)| {
            *acc.entry(right).or_insert(0) += 1;
            acc
        });

    pairs
        .iter()
        .map(|&(left, _)| left * counts.get(&left).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part_1(input), 11);
        assert_eq!(part_2(input), 31);
    }
}
