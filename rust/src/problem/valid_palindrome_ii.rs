pub fn valid_palindrome_ii(s: String) -> bool {
    fn is_palindrome(s: &[u8], k: i32) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            if s[i] != s[j] {
                return if k == 0 {
                    false
                } else {
                    is_palindrome(&s[i + 1..=j], k - 1) || is_palindrome(&s[i..j], k - 1)
                };
            }

            i += 1;
            j -= 1;
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
        assert!(!valid_palindrome_ii("abc".to_string()));
    }
}