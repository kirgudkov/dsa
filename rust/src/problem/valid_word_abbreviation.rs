pub fn valid_word_abbreviation2(word: String, abbr: String) -> bool {
    let mut i = 0;
    let mut j = 0;

    let word_bytes = word.as_bytes();
    let abbr_bytes = abbr.as_bytes();

    while i < word_bytes.len() && j < abbr_bytes.len() {
        if word_bytes[i] == abbr_bytes[j] {
            i += 1;
            j += 1;
            continue;
        }

        if abbr_bytes[j] == b'0' || !abbr_bytes[j].is_ascii_digit() {
            return false;
        }

        let k = j;
        while j < abbr_bytes.len() && abbr_bytes[j].is_ascii_digit() {
            j += 1;
        }

        i += abbr[k..j].parse::<usize>().unwrap();
    }

    i == word.len() && j == abbr.len()
}

pub fn valid_word_abbreviation1(word: String, abbr: String) -> bool {
    let abbr_bytes = abbr.as_bytes();

    let mut unwrapped = String::new();
    let mut i = 0;

    while i < abbr_bytes.len() {
        if abbr_bytes[i].is_ascii_digit() {
            if abbr_bytes[i] == b'0' {
                return false;
            }

            let mut j = i + 1;
            while j < abbr_bytes.len() && abbr_bytes[j].is_ascii_digit() {
                j += 1;
            }

            let count = abbr[i..j].parse::<usize>().unwrap();

            if count > word.len() {
                return false;
            }

            unwrapped.push_str("#".repeat(count).as_str());
            i = j;
        } else {
            unwrapped.push(abbr_bytes[i] as char);
            i += 1;
        }
    }

    unwrapped.len() == word.len() && unwrapped.chars().zip(word.chars()).all(|(a, w)| {
        a == w || a == '#'
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_word_abbreviation1() {
        assert!(!valid_word_abbreviation1("bignumberhahaha".to_string(), "999999999".to_string()));
        assert!(valid_word_abbreviation1("internationalization".to_string(), "i12iz4n".to_string()));
        assert!(valid_word_abbreviation1("internationalization".to_string(), "i5a11o1".to_string()));
        assert!(!valid_word_abbreviation1("apple".to_string(), "a2e".to_string()));
        assert!(valid_word_abbreviation1("apple".to_string(), "a3e".to_string()));
        assert!(!valid_word_abbreviation1("apple".to_string(), "a4e".to_string()));
    }

    #[test]
    fn test_valid_word_abbreviation2() {
        assert!(!valid_word_abbreviation2("bignumberhahaha".to_string(), "999999999".to_string()));
        assert!(valid_word_abbreviation2("internationalization".to_string(), "i12iz4n".to_string()));
        assert!(valid_word_abbreviation2("internationalization".to_string(), "i5a11o1".to_string()));
        assert!(!valid_word_abbreviation2("apple".to_string(), "a2e".to_string()));
    }
}