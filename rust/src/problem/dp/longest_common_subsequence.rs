pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp = vec![vec![0; text1.len() + 1]; text2.len() + 1];

    for i in (0..text2.len()).rev() {
        for j in (0..text1.len()).rev() {
            if text2.as_bytes()[i] == text1.as_bytes()[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
        assert_eq!(longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
        assert_eq!(longest_common_subsequence("gxtxayb".to_string(), "aggtab".to_string()), 4);
        assert_eq!(longest_common_subsequence("tggatg".to_string(), "taggat".to_string()), 5);
    }
}