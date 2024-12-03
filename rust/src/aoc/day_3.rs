fn part_1(input: &str) -> i32 {
    let mut result = 0;
    let chars = input.chars().collect::<Vec<_>>();

    for (mut i, pat) in input.match_indices("mul(") {
        i += pat.len();

        // Parse lhs
        let mut j = i;
        while chars[j].is_ascii_digit() {
            j += 1;
        }
        if chars[j] != ',' {
            // Ensure delimiter is correct
            continue;
        }
        let a = input[i..j].parse::<i32>().unwrap();

        // Parse rhs
        i = j + 1;
        j = i;
        while chars[j].is_ascii_digit() {
            j += 1;
        }
        if chars[j] != ')' {
            // Ensure closing bracket
            continue;
        }
        let b = input[i..j].parse::<i32>().unwrap();

        // Pattern matched: update result
        result += a * b;
    }

    result
}

fn part_2(input: &str) -> i32 {
    let end = input.find("don't()").unwrap_or(input.len());

    let rest = match input[end..].find("do()") {
        Some(next) => part_2(&input[end + next + 4..]),
        None => 0,
    };

    part_1(&input[..end]) + rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(input), 161);
        dbg!(part_1(&std::fs::read_to_string("./src/aoc/inputs/day_3").unwrap()));
    }

    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(input), 48);
        dbg!(part_2(&std::fs::read_to_string("./src/aoc/inputs/day_3").unwrap()));
    }
}
