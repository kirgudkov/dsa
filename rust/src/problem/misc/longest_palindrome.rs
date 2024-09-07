use std::ops::RangeInclusive;

// https://leetcode.com/problems/longest-palindromic-substring/
pub fn longest_palindrome(s: String) -> String {
    let mut range = RangeInclusive::new(0, 0);
    let bytes = s.as_bytes();

    for i in 0..bytes.len() * 2 {
        let mut l = i as i32 / 2;
        let mut r = i / 2 + if i % 2 == 0 { 0 } else { 1 };

        while l >= 0 && r < bytes.len() && bytes[l as usize] == bytes[r] {
            if r - l as usize > range.end() - range.start() {
                range = RangeInclusive::new(l as usize, r);
            }

            l -= 1;
            r += 1;
        }
    }

    s[range].to_string()
}

fn longest_palindrome_n2(s: String) -> String {
    let bytes = s.as_bytes();
    let len = s.len() as i32;

    let expand = |mut i: i32, mut j: i32| -> (i32, i32) {
        while i >= 0 && j < len && bytes[i as usize] == bytes[j as usize] {
            i -= 1;
            j += 1;
        }

        (i + 1, j - 1)
    };

    let mut ans = (0, 0);

    for i in 0..len {
        let (l1, r1) = expand(i, i);
        let (l2, r2) = expand(i, i + 1);

        if r1 - l1 > ans.1 - ans.0 {
            ans = (l1, r1)
        }

        if r2 - l2 > ans.1 - ans.0 {
            ans = (l2, r2)
        }
    }

    s[ans.0 as usize..=ans.1 as usize].to_string()
}

// Dynamic programming approach O(n^2)
fn longest_palindrome_dp(s: String) -> String {
    let bytes = s.as_bytes();
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut ans = (0, 0);

    // Fill diagonal with true because strings with length 1 are always palindromes
    for i in 0..s.len() {
        dp[i][i] = true;
    }

    // Fill nearest diagonal to check even-length palindromes (2 consecutive chars)
    for i in 0..s.len() - 1 {
        if bytes[i] == bytes[i + 1] {
            dp[i][i + 1] = true;
            ans = (i, i + 1);
        }
    }

    for step in 2..s.len() {
        for i in 0..s.len() - step {
            let j = i + step;

            if bytes[i] == bytes[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                ans = (i, j);
            }
        }
    }

    s[ans.0..=ans.1].to_string()
}

// Brute force solution O(n^3)
fn longest_palindrome_bf(s: String) -> String {
    let bytes = s.as_bytes();
    // O(n^2)
    for r in (0..s.len()).rev() {
        for l in 0..=s.len() - r {
            if is_palindrome(bytes, l, r) {
                return s[l..=r].to_string();
            }
        }
    }

    // O(n)
    fn is_palindrome(s: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if s[l] != s[r] {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb".to_string());
        assert_eq!(longest_palindrome(String::from("alevelevelqweytty")), "levelevel".to_string());
        assert_eq!(longest_palindrome(String::from("aaabaaaa")), "aaabaaa".to_string());
    }

    #[test]
    fn test_longest_palindrome_bf() {
        assert_eq!(longest_palindrome_bf(String::from("cbbd")), "bb".to_string());
        assert_eq!(longest_palindrome_bf(String::from("alevelevelqweytty")), "levelevel".to_string());
        assert_eq!(longest_palindrome_bf(String::from("aaabaaaa")), "aaabaaa".to_string());
    }

    #[test]
    fn test_longest_palindrome_dp() {
        assert_eq!(longest_palindrome_dp(String::from("cbbd")), "bb".to_string());
        assert_eq!(longest_palindrome_dp(String::from("alevelevelqweytty")), "levelevel".to_string());
        assert_eq!(longest_palindrome_dp(String::from("aaabaaaa")), "aaabaaa".to_string());
    }

    #[test]
    fn test_longest_palindrome_n2() {
        assert_eq!(longest_palindrome_n2(String::from("cbbd")), "bb".to_string());
        assert_eq!(longest_palindrome_n2(String::from("alevelevelqweytty")), "levelevel".to_string());
        assert_eq!(longest_palindrome_n2(String::from("aaabaaaa")), "aaabaaa".to_string());
    }
}
