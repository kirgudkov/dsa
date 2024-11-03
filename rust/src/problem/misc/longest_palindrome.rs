// https://leetcode.com/problems/longest-palindromic-substring/
pub fn longest_palindrome(s: String) -> String {
    let vec = s.chars().collect::<Vec<_>>();
    let mut range = (0, 0);

    let expand = |mut l: i32, mut r: i32| -> (i32, i32) {
        while l >= 0 && r < s.len() as i32 && vec[l as usize] == vec[r as usize] {
            l -= 1;
            r += 1;
        }

        (l + 1, r - 1)
    };

    for i in 0..s.len() as i32 * 2 {
        let (l, r) = expand(i / 2, if i % 2 == 0 { i / 2 } else { i / 2 + 1 });

        if r - l > range.1 - range.0 {
            range = (l, r)
        }
    }

    s[range.0 as usize..=range.1 as usize].to_string()
}

fn longest_palindrome_n2(s: String) -> String {
    let vec = s.chars().collect::<Vec<_>>();
    let mut range = (0, 0);

    let expand = |mut i: i32, mut j: i32| -> (i32, i32) {
        while i >= 0 && j < vec.len() as i32 && vec[i as usize] == vec[j as usize] {
            i -= 1;
            j += 1;
        }

        (i + 1, j - 1)
    };

    for i in 0..vec.len() as i32 {
        let (l1, r1) = expand(i, i);
        let (l2, r2) = expand(i, i + 1);

        if r1 - l1 > range.1 - range.0 {
            range = (l1, r1)
        }

        if r2 - l2 > range.1 - range.0 {
            range = (l2, r2)
        }
    }

    s[range.0 as usize..=range.1 as usize].to_string()
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
    let vec = s.chars().collect::<Vec<_>>();

    // O(n)
    let is_palindrome = |mut l: usize, mut r: usize| -> bool {
        while l < r && vec[l] == vec[r] {
            l += 1;
            r -= 1;
        }

        l >= r
    };

    // O(n^2)
    for r in (0..s.len()).rev() {
        for l in 0..=s.len() - r {
            if is_palindrome(l, r) {
                return s[l..=r].to_string();
            }
        }
    }

    unreachable!()
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
