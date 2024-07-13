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

        // We only can go further if the current abbreviation char is a digit and it is not zero 
        if abbr_bytes[j].is_ascii_digit() && abbr_bytes[j] <= b'0' || abbr_bytes[j] > b'9' {
            return false;
        }

        // Find the end of the number
        let k = j;
        while j < abbr_bytes.len() && abbr_bytes[j].is_ascii_digit() {
            j += 1;
        }
        // Move the word pointer by the number of characters
        i += abbr[k..j].to_string().parse::<usize>().unwrap();
    }

    // If we reached the end of both strings, then the abbreviation is valid
    i == word.len() && j == abbr.len()
}

pub fn valid_word_abbreviation1(word: String, abbr: String) -> bool {
    let mut unwrapped = String::new();

    let mut i = 0;
    let abbr_bytes = abbr.as_bytes();

    while i < abbr_bytes.len() {
        if abbr_bytes[i].is_ascii_digit() {
            // Leading zeros are not allowed
            if abbr_bytes[i] == b'0' {
                return false;
            }

            // Find the end of the number
            let mut j = i + 1;
            while j < abbr_bytes.len() && abbr_bytes[j].is_ascii_digit() {
                j += 1;
            }

            // Parse the number
            let count = abbr[i..j].to_string().parse::<usize>().unwrap();

            // Number should not be greater than the length of the word
            if count > word.len() {
                return false;
            }

            // Unwrap placeholder
            unwrapped.push_str("#".repeat(count).as_str());
            i = j;
        } else {
            unwrapped.push(abbr_bytes[i] as char);
            i += 1;
        }
    }

    // At this point we should have unwrapped string and word of the same length:
    // internationalization
    // i############iz####n

    // If the lengths are different, then the abbreviation is invalid
    if unwrapped.len() != word.len() {
        return false;
    }

    i = 0;
    let abr = unwrapped.as_bytes();
    let word = word.as_bytes();

    while i < abr.len() {
        if abr[i] == b'#' {
            i += 1;
            continue;
        }

        if abr[i] != word[i] {
            return false;
        }

        i += 1;
    }

    true
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
    }

    #[test]
    fn test_valid_word_abbreviation2() {
        assert!(!valid_word_abbreviation2("bignumberhahaha".to_string(), "999999999".to_string()));
        assert!(valid_word_abbreviation2("internationalization".to_string(), "i12iz4n".to_string()));
        assert!(valid_word_abbreviation2("internationalization".to_string(), "i5a11o1".to_string()));
        assert!(!valid_word_abbreviation2("apple".to_string(), "a2e".to_string()));
    }
}