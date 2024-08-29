// https://leetcode.com/problems/maximum-score-words-formed-by-letters
pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut bt = Bt::new(words, letters, score);
    bt.backtrack(0);
    bt.max_score
}

struct Bt {
    words: Vec<String>,
    freqs: Vec<usize>,
    score: Vec<i32>,
    current_score: i32,
    max_score: i32,
    current_letters: Vec<usize>,
}

impl Bt {
    fn new(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> Self {
        let freqs = letters.iter().fold(vec![0usize; 26], |mut vec, &c| {
            vec[(c as u8 - b'a') as usize] += 1;
            vec
        });

        Self {
            words,
            freqs,
            score,
            current_score: 0,
            max_score: i32::MIN,
            current_letters: vec![0; 26],
        }
    }

    fn backtrack(&mut self, i: usize) {
        if i == self.words.len() {
            self.max_score = self.max_score.max(self.current_score);
            return;
        }

        // skip word
        self.backtrack(i + 1);

        // count word
        for &c in self.words[i].as_bytes() {
            self.current_letters[(c - b'a') as usize] += 1;
            self.current_score += self.score[(c - b'a') as usize]
        }

        let is_valid = self.words[i].as_bytes().iter().all(|&c| {
            self.freqs[(c - b'a') as usize] >= self.current_letters[(c - b'a') as usize]
        });

        // if there is not enough letter, skip word
        if is_valid {
            self.backtrack(i + 1);
        }

        // in any case: rollback
        for &c in self.words[i].as_bytes() {
            self.current_letters[(c - b'a') as usize] -= 1;
            self.current_score -= self.score[(c - b'a') as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::backtracking::max_score_words_1255::max_score_words;

    #[test]
    fn test_max_score_words() {
        assert_eq!(
            max_score_words(
                vec!["dog".to_string(), "cat".to_string(), "dad".to_string(), "good".to_string()],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ), 23
        );
        assert_eq!(
            max_score_words(
                vec!["xxxz".to_string(), "ax".to_string(), "bx".to_string(), "cx".to_string()],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10],
            ), 27
        );
        assert_eq!(
            max_score_words(
                vec!["leetcode".to_string()],
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            ), 0
        );
        assert_eq!(
            max_score_words(
                vec!["ac".to_string(), "abd".to_string(), "db".to_string(), "ba".to_string(), "dddd".to_string(), "bca".to_string()],
                vec!['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c', 'd', 'd', 'd', 'd'],
                vec![6, 4, 4, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ), 62
        );
    }
}
