pub fn first_uniq_char(s: String) -> i32 {
    let mut map = std::collections::HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for (i, c) in s.chars().enumerate() {
        if *map.get(&c).unwrap() == 1 {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(first_uniq_char("aabb".to_string()), -1);
    }
}