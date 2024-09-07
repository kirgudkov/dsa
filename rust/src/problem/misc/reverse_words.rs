use std::mem::take;

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

// idiomatic rust version
fn reverse_words_1(s: String) -> String {
    let mut result = String::new();
    let mut word = String::new();

    for c in s.chars().chain([' ']) {
        match c {
            ' ' => if !word.is_empty() {
                result = format!("{}{}{}", take(&mut word), if result.is_empty() { "" } else { " " }, result);
            }
            _ => word.push(c),
        }
    }

    result
}

// even more idiomatic rust version
fn reverse_words_2(s: String) -> String {
    s.chars().chain([' ']).fold((String::new(), String::new()), |(result, mut word), c| {
        match c {
            ' ' if !word.is_empty() => {
                (format!("{}{}{}", take(&mut word), if result.is_empty() { "" } else { " " }, result), word)
            }
            ' ' => {
                (result, word)
            }
            _ => {
                (result, format!("{}{}", word, c))
            }
        }
    }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("  hello       world  ".to_string()), "world hello".to_string());
        assert_eq!(reverse_words("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(reverse_words("".to_string()), "".to_string());
        assert_eq!(reverse_words(" ".to_string()), "".to_string());
    }

    #[test]
    fn test_reverse_words_1() {
        assert_eq!(reverse_words_1("  hello       world  ".to_string()), "world hello".to_string());
        assert_eq!(reverse_words_1("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(reverse_words_1("".to_string()), "".to_string());
        assert_eq!(reverse_words_1(" ".to_string()), "".to_string());
        assert_eq!(reverse_words_1("Let's take LeetCode contest".to_string()), "contest LeetCode take Let's".to_string());
    }

    #[test]
    fn test_reverse_words_2() {
        assert_eq!(reverse_words_2("  hello       world  ".to_string()), "world hello".to_string());
        assert_eq!(reverse_words_2("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(reverse_words_2("".to_string()), "".to_string());
        assert_eq!(reverse_words_2(" ".to_string()), "".to_string());
        assert_eq!(reverse_words_2("Let's take LeetCode contest".to_string()), "contest LeetCode take Let's".to_string());
    }
}
