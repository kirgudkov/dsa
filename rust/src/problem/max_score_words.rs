use std::collections::HashMap;

// In progress, last test case fails.
// Need to learn more about backtracking

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut counts: Vec<i32> = vec![0; 26];
    let mut scores: Vec<i32> = vec![];

    for letter in letters.iter() {
        counts[*letter as usize - 'a' as usize] += 1;
    }

    let mut i = 0;
    while i < words.len() {
        let set = build_words(&words, counts.clone(), i);
        scores.push(calculate_score(&set, &score));

        i += 1;
    }

    *scores.iter().max().unwrap_or(&0)
}

fn build_words<'a>(
    words: &'a [String], mut counts: Vec<i32>, mut from: usize,
) -> Vec<(&'a String, HashMap<char, i32>)> {
    let mut result: Vec<(&'a String, HashMap<char, i32>)> = Vec::new();

    while from < words.len() {
        if let Some(word) = build_word(&words[from], &mut counts) {
            result.push(word);
        }

        from += 1;
    }

    result
}

fn build_word<'a>(word: &'a String, counts: &mut [i32]) -> Option<(&'a String, HashMap<char, i32>)> {
    let mut freq: HashMap<char, i32> = HashMap::new();

    for char in word.chars() {
        let entry = freq.entry(char).or_insert(0);
        *entry += 1;
    }

    for (char, needed) in freq.iter() {
        if counts[*char as usize - 'a' as usize] < *needed {
            return None;
        }
    }

    for (char, needed) in freq.iter() {
        counts[*char as usize - 'a' as usize] -= needed;
    }

    Some((word, freq))
}

fn calculate_score(set: &[(&String, HashMap<char, i32>)], score_map: &[i32]) -> i32 {
    let mut score = 0;

    for (_, freq) in set.iter() {
        let mut word_score = 0;

        for (char, count) in freq.iter() {
            word_score += score_map[*char as usize - 'a' as usize] * count;
        }

        score += word_score;
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::problem::max_score_words::max_score_words;

    #[test]
    fn test_max_score_words() {
        assert_eq!(
            max_score_words(
                ["dog", "cat", "dad", "good"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            23
        );
        assert_eq!(
            max_score_words(
                ["xxxz", "ax", "bx", "cx"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10],
            ),
            27
        );
        assert_eq!(
            max_score_words(
                ["leetcode"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            ),
            0
        );
    }
}
