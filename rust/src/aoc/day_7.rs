fn part_1(input: &str, ops: &[Ops]) -> i64 {
    parse(input)
        .iter()
        .filter(|(target, nums)| dfs(target, nums, nums[0], 1, ops))
        .map(|(target, _)| *target)
        .sum()
}

enum Ops {
    Sum,
    Multiply,
    Concatenate,
}

impl Ops {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Ops::Sum => a + b,
            Ops::Multiply => a * b,
            Ops::Concatenate => format!("{}{}", a, b).parse::<i64>().expect("Invalid number"),
        }
    }
}

fn dfs(target: &i64, nums: &[i64], acc: i64, i: usize, ops: &[Ops]) -> bool {
    if i == nums.len() {
        return acc == *target;
    }

    for op in ops {
        if dfs(target, nums, op.apply(acc, nums[i]), i + 1, ops) {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            let target = line.next().unwrap().trim_end_matches(':').parse().unwrap();
            let nums = line.map(|x| x.parse().unwrap()).collect();
            (target, nums)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part_1(input, &[Ops::Sum, Ops::Multiply]), 3749);
        dbg!(part_1(
            &std::fs::read_to_string("./src/aoc/inputs/day_7").unwrap(),
            &[Ops::Sum, Ops::Multiply]
        ));

        assert_eq!(part_1(input, &[Ops::Sum, Ops::Multiply, Ops::Concatenate]), 11387);
        dbg!(part_1(
            &std::fs::read_to_string("./src/aoc/inputs/day_7").unwrap(),
            &[Ops::Sum, Ops::Multiply, Ops::Concatenate]
        ));
    }
}
