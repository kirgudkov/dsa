fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for ch in s.chars() {
        match ch {
            '{' | '(' | '[' => stack.push(ch),
            _ => if !matches!((stack.pop(), ch), (Some('{'), '}') | (Some('('), ')') | (Some('['), ']')) {
                return false;
            }
        }
    }

    stack.is_empty()
}

// fn is_valid(s: String) -> bool {
//     let mut stack: Vec<char> = vec![];
// 
//     s.chars().try_for_each(|c| {
//         match c {
//             '{' | '(' | '[' => stack.push(c),
//             _ => if !matches!((stack.pop(), c), (Some('{'), '}') | (Some('('), ')') | (Some('['), ']')) {
//                 return Err(())
//             }
//         };
// 
//         Ok(())
//     }).is_ok() && stack.is_empty()
// }

#[cfg(test)]
mod tests {
    use crate::problem::valid_parentheses::is_valid;

    #[test]
    fn test_1() {
        assert!(!is_valid(")".to_string()));
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(is_valid("{[]}".to_string()));
    }
}
