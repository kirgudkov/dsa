pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = vec![];

    fn backtrack(result: &mut Vec<String>, buf: &mut Vec<char>, open: i32, close: i32) {
        if open == 0 && close == 0 {
            result.push(buf.iter().collect());
            return;
        }

        if open > 0 {
            buf.push('(');
            backtrack(result, buf, open - 1, close);
            buf.pop();
        }

        if open < close {
            buf.push(')');
            backtrack(result, buf, open, close - 1);
            buf.pop();
        }
    }

    backtrack(&mut result, &mut vec![], n, n);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(generate_parenthesis(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
