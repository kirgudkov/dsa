use std::collections::HashMap;

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

// This is more feasible solution. Runs in O(M * N), where N is the s length and M is the dict size.
// Without memoization it performs poorly: O(M^N)
pub fn word_break_rec(s: String, word_dict: Vec<String>) -> bool {
    fn test(s: &str, dict: &Vec<String>, memo: &mut HashMap<String, bool>) -> bool {
        // Base case - s is consumed, all "puzzles" were found
        if s.is_empty() {
            return true;
        }

        if let Some(&result) = memo.get(s) {
            return result;
        }

        // Since each word in dict can be used multiple times, we're obliged to run the entire array
        for word in dict {
            if s.starts_with(word) && test(&s[word.len()..], dict, memo) {
                memo.insert(s.to_string(), true);
                return true;
            }
        }

        memo.insert(s.to_string(), false);
        false
    }

    test(&s, &word_dict, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
        assert!(word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));
        assert!(!word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]));

        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert!(word_break_rec(s, word_dict));

        let s = "catsandog".to_string();
        let word_dict = vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()];
        assert!(!word_break_rec(s, word_dict));

        let s = "applepenapple".to_string();
        let word_dict = vec!["app".to_string(), "apple".to_string(), "pen".to_string()];
        assert!(word_break_rec(s, word_dict));
    }
}