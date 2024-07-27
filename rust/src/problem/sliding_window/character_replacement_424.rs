// https://leetcode.com/problems/longest-repeating-character-replacement/
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut map = std::collections::HashMap::with_capacity(s.len());
    let bytes = s.as_bytes();

    let mut l = 0;
    let mut max_freq = 0;
    let mut res = i32::MIN;

    for (r, b) in bytes.iter().enumerate() {
        let freq = map.entry(b).or_insert(0);
        *freq += 1;
        max_freq = max_freq.max(*freq);

        if (r - l + 1) as i32 - max_freq > k {
            *map.get_mut(&bytes[l]).unwrap() -= 1;
            l += 1;
        }

        res = res.max((r - l + 1) as i32);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}