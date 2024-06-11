pub fn reverse_words(s: String) -> String {
    let mut res = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < s.len() {
        while i < s.len() && bytes[i] == b' ' {
            i += 1;
        }

        let mut word = String::new();

        while i < s.len() && bytes[i] != b' ' {
            word.push(bytes[i] as char);
            i += 1;
        }

        if !word.is_empty() {
            res = if res.is_empty() {
                word
            } else {
                format!("{} {}", word, res)
            };
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse_words("  hello       world  ".to_string()), "world hello".to_string());
        assert_eq!(reverse_words("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(reverse_words("".to_string()), "".to_string());
        assert_eq!(reverse_words(" ".to_string()), "".to_string());
    }
}
