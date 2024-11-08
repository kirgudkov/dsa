pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    if let Ok(num) = expression.parse::<i32>() {
        return vec![num];
    }

    let mut result = vec![];

    for (i, char) in expression.chars().enumerate() {
        match char {
            '+' | '-' | '*' => {
                for l in diff_ways_to_compute(expression[..i].to_string()) {
                    for r in diff_ways_to_compute(expression[i + 1..].to_string()) {
                        match char {
                            '+' => result.push(l + r),
                            '-' => result.push(l - r),
                            '*' => result.push(l * r),
                            _ => unreachable!()
                        }
                    }
                }
            }
            _ => continue
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(diff_ways_to_compute("2-1-1".to_string()), vec![2, 0]);
        assert_eq!(diff_ways_to_compute("2*3-4*5".to_string()), vec![-34, -10, -14, -10, 10]);
    }
}