use std::ops::RangeInclusive;

// https://leetcode.com/problems/longest-palindromic-substring/
pub fn longest_palindrome(s: String) -> String {
    let mut range = RangeInclusive::new(0, 0);
    let chars = s.as_bytes();

    for i in 0..chars.len() * 2 {
        let mut l = i as i32 / 2;
        let mut r = i / 2 + if i % 2 == 0 { 0 } else { 1 };

        while r < chars.len() && l >= 0 {
            if chars[l as usize] != chars[r] {
                break;
            }

            if r - l as usize > range.end() - range.start() {
                range = RangeInclusive::new(l as usize, r);
            }

            l -= 1;
            r += 1;
        }
    }

    s[range].to_string()
}

#[cfg(test)]
mod tests {
    use crate::problem::longest_palindrome::longest_palindrome;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb".to_string());
        assert_eq!(longest_palindrome(String::from("alevelevelqweytty")), "levelevel".to_string());
        assert_eq!(longest_palindrome(String::from("aaabaaaa")), "aaabaaa".to_string());
    }
}
