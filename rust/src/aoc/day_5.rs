use std::collections::HashMap;

fn part_1(input: &str) -> i32 {
    let order_rules = parse_order_rules(input);

    parse_rows(input)
        .iter()
        // Filter out invalid rows
        .filter(|row| {
            row.windows(2)
                // row is valid if each entry is known and each pair follows order rules
                .all(|pair| order_rules.contains_key(&pair[0]) && order_rules[&pair[0]].contains(&pair[1]))
        })
        // Sum their middle elements
        .fold(0, |sum, row| sum + row[row.len() / 2])
}

fn part_2(input: &str) -> i32 {
    let order_rules = parse_order_rules(input);

    parse_rows(input)
        .iter_mut()
        // Filter out valid rows
        .filter(|row| {
            row.windows(2)
                // row is invalid if any entry is unknown or any pair does not follow order rules
                .any(|pair| !order_rules.contains_key(&pair[0]) || !order_rules[&pair[0]].contains(&pair[1]))
        })
        .map(|row| {
            // Sort them by the order rules using custom comparator function
            row.sort_unstable_by(|a, b| {
                if order_rules.contains_key(a) && order_rules[a].contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            // return mid item and sum
            row[row.len() / 2]
        })
        .sum()
}

fn parse_rows(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn parse_order_rules(input: &str) -> HashMap<i32, Vec<i32>> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split("|").map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut acc, pair| {
            acc.entry(pair[0]).or_default().push(pair[1]);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part_1(input), 143);
        dbg!(part_1(&std::fs::read_to_string("./src/aoc/inputs/day_5").unwrap()));
        assert_eq!(part_2(input), 123);
        dbg!(part_2(&std::fs::read_to_string("./src/aoc/inputs/day_5").unwrap()));
    }
}
