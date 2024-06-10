pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let mut set = std::collections::HashSet::new();
    let mut res = 0;

    let mut l = 0;
    let mut r = 0;
    let bytes = s.as_bytes();

    while r < s.len() {
        if set.contains(&bytes[r]) {
            res = res.max(r - l);
            set.remove(&bytes[l]);
            l += 1;

            continue;
        }

        set.insert(bytes[r]);
        r += 1;
    }

    res = res.max(r - l);

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }
}