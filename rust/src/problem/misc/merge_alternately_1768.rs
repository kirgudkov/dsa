use std::iter::zip;

// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let w1_chars = word1.chars().collect::<Vec<_>>();
//     let w2_chars = word2.chars().collect::<Vec<_>>();
// 
//     let mut result = String::new();
// 
//     for i in 0..w1_chars.len() + w2_chars.len() {
//         if i < word1.len() { result.push(w1_chars[i]); }
//         if i < word2.len() { result.push(w2_chars[i]); }
//     }
// 
//     result
// }

pub fn merge_alternately(word1: String, word2: String) -> String {
    let min_len = word1.len().min(word2.len());
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    let zipped = zip(
        chars1.by_ref().take(min_len),
        chars2.by_ref().take(min_len),
    );

    let str = zipped.fold(String::new(), |acc, (a, b)| {
        format!("{}{}{}", acc, a, b)
    });

    format!("{}{}{}", str, chars1.collect::<String>(), chars2.collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr".to_string());
        assert_eq!(merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd".to_string());
        assert_eq!(merge_alternately("ab".to_string(), "pqrs".to_string()), "apbqrs".to_string());
    }
}