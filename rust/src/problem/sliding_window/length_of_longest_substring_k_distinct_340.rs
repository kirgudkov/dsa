// https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/
pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    let mut freqs = std::collections::HashMap::with_capacity(s.len());
    let mut max = i32::MIN;
    let mut l = 0;
    let bytes = s.as_bytes();

    for (r, b) in bytes.iter().enumerate() {
        *freqs.entry(b).or_insert(0) += 1;

        while freqs.len() > k as usize {
            freqs.entry(&bytes[l]).and_modify(|e| *e -= 1);

            if *freqs.get(&bytes[l]).unwrap() == 0 {
                freqs.remove(&bytes[l]);
            }

            l += 1;
        }

        max = max.max((r - l + 1) as i32);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(length_of_longest_substring_k_distinct("eceba".to_string(), 2), 3);
        assert_eq!(length_of_longest_substring_k_distinct("aa".to_string(), 1), 2);
        assert_eq!(length_of_longest_substring_k_distinct("aba".to_string(), 1), 1);
    }
}