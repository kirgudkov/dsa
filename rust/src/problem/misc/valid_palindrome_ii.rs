pub fn valid_palindrome_ii(s: String) -> bool {
    fn is_palindrome(s: &[u8], k: i32) -> bool {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            if s[l] != s[r] {
                return if k == 0 {
                    false
                } else {
                    is_palindrome(&s[l + 1..=r], k - 1) || is_palindrome(&s[l..r], k - 1)
                };
            }

            l += 1;
            r -= 1;
        }

        true
    }

    is_palindrome(s.as_bytes(), 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome_ii() {
        assert!(valid_palindrome_ii("aba".to_string()));
        assert!(valid_palindrome_ii("abca".to_string()));
        assert!(valid_palindrome_ii("abfggba".to_string()));
        assert!(!valid_palindrome_ii("abfca".to_string()));
        assert!(!valid_palindrome_ii("abc".to_string()));
    }
}