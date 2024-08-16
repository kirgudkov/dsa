// https://leetcode.com/problems/find-common-characters
pub fn common_chars(words: Vec<&str>) -> Vec<String> {
    let first_word_freq = words[0].as_bytes().iter()
        .fold(vec![0; 26], |mut freq, b| {
            freq[(b - b'a') as usize] += 1;
            freq
        });

    words.iter().skip(1)
        .fold(first_word_freq, |mut prev_freq, word| {
            word.as_bytes().iter().fold(vec![0; 26], |mut next_freq, b| {
                if prev_freq[(b - b'a') as usize] > 0 {
                    prev_freq[(b - b'a') as usize] -= 1;
                    next_freq[(b - b'a') as usize] += 1;
                }

                next_freq
            })
        })
        .iter().enumerate()
        .flat_map(|(i, &count)| {
            std::iter::repeat(((b'a' + i as u8) as char).to_string()).take(count)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut words = common_chars(vec!["bella", "label", "roller"]);
        let mut expected = vec!["l", "l", "e"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);

        let mut words = common_chars(vec!["cool", "lock", "cook"]);
        let mut expected = vec!["o", "c"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);

        let mut words = common_chars(vec!["cool"]);
        let mut expected = vec!["c", "o", "o", "l"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);
    }
}
