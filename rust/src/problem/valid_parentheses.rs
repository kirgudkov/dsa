pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for next in s.chars() {
        match next {
            '{' | '(' | '[' => stack.push(next),
            _ => {
                if let Some(last) = stack.pop() {
                    if !matches!((last, next), ('{', '}') | ('(', ')') | ('[', ']')) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::problem::valid_parentheses::is_valid;

    #[test]
    fn test_1() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(is_valid("{[]}".to_string()));
    }
}
