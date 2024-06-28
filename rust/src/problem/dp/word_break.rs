pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len()];

    for i in 0..s.len() {
        for word in &word_dict {
            if i + 1 >= word.len() && &s[i + 1 - word.len()..=i] == word {
                dp[i] = dp[i] || (i + 1 - word.len() == 0 || dp[i - word.len()]);
            }
        }
    }

    *dp.last().unwrap_or(&false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
        assert!(word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));
        assert!(!word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]));
    }
}