pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }

    for i in 0..haystack.len() {
        if haystack[i.saturating_sub(needle.len() - 1)..=i] == needle {
            return i as i32 - needle.len() as i32 + 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_str("sadbusabutsad".to_string(), "but".to_string()), 7);
        assert_eq!(str_str("".to_string(), "".to_string()), 0);
    }
}
