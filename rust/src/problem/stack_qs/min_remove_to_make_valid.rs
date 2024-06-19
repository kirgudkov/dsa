// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses
// Time Complexity: O(n)
// Space Complexity: O(n)
pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = Vec::new();
    let mut chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '(' => {
                stack.push(chars[i]);
                i += 1;
            }
            ')' => {
                if stack.last().is_some() {
                    stack.pop();
                    i += 1;
                } else {
                    chars.remove(i);
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    stack.clear();
    let mut i = chars.len() as i32 - 1;
    while i >= 0 {
        match chars[i as usize] {
            ')' => {
                stack.push(chars[i as usize]);
            }
            '(' => {
                if stack.last().is_some() {
                    stack.pop();
                } else {
                    chars.remove(i as usize);
                }
            }
            _ => {}
        }

        i -= 1;
    }


    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_remove_to_make_valid() {
        assert_eq!(min_remove_to_make_valid("lee(t(c)o)de)".to_string()), "lee(t(c)o)de".to_string());
        assert_eq!(min_remove_to_make_valid("a)b(c)d".to_string()), "ab(c)d".to_string());
        assert_eq!(min_remove_to_make_valid("))((".to_string()), "".to_string());
        assert_eq!(min_remove_to_make_valid("(a(b(c)d)".to_string()), "a(b(c)d)".to_string());
        assert_eq!(min_remove_to_make_valid("".to_string()), "".to_string());
        assert_eq!(min_remove_to_make_valid("a(b(c)d".to_string()), "ab(c)d".to_string());
    }
}
