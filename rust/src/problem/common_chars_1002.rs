// https://leetcode.com/problems/find-common-characters
fn common_chars(words: Vec<&str>) -> Vec<String> {
    // 1. Count chars in the first word;
    let mut prev = [0; 26];

    for b in words[0].as_bytes() {
        prev[(b - b'a') as usize] += 1;
    }

    // 2. For each next word we do the same but count only those chars that appeared previously;
    for word in words.iter().skip(1) {
        let mut curr = [0; 26];

        for b in word.as_bytes() {
            let i = (b - b'a') as usize;

            if prev[i] > 0 {
                prev[i] -= 1;
                curr[i] += 1;
            }
        }

        // current freq map becomes prev for the next word
        prev = curr;
    }

    // 3. Build result vector 
    let mut result = vec![];

    for (i, &count) in prev.iter().enumerate() {
        for _ in 0..count {
            result.push(((b'a' + i as u8) as char).to_string());
        }
    }

    result
}

// This code does exactly the same as the above one but rather in declarative way;
fn _common_chars(words: Vec<&str>) -> Vec<String> {
    let first_word_freq = words[0].as_bytes().iter()
        .fold(vec![0; 26], |mut freq, b| {
            freq[(b - b'a') as usize] += 1;
            freq
        });

    words.iter().skip(1)
        .fold(first_word_freq, |mut prev, word| {
            word.as_bytes().iter().fold(vec![0; 26], |mut curr, b| {
                let i = (b - b'a') as usize;

                if prev[i] > 0 {
                    prev[i] -= 1;
                    curr[i] += 1;
                }

                curr
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
        let mut words = _common_chars(vec!["bella", "label", "roller"]);
        let mut expected = vec!["l", "l", "e"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);

        let mut words = _common_chars(vec!["cool", "lock", "cook"]);
        let mut expected = vec!["o", "c"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);

        let mut words = _common_chars(vec!["cool"]);
        let mut expected = vec!["c", "o", "o", "l"];

        words.sort_unstable();
        expected.sort_unstable();
        assert_eq!(words, expected);
    }

    #[test]
    fn test_2() {
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
