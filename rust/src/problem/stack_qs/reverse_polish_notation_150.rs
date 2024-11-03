// https://leetcode.com/problems/evaluate-reverse-polish-notation
fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];

    for token in tokens {
        if let Ok(num) = token.parse() {
            stack.push(num);
        } else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();

            stack.push(match token.as_str() {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                _ => unreachable!()
            });
        }
    }

    stack[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]), 9);
        assert_eq!(eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]), 6);
        assert_eq!(eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()]), 22);
        assert_eq!(eval_rpn(vec!["18".to_string()]), 18);
    }
}
