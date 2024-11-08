use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;

// N - word_list length
// M - max word length
// Overall time complexity: O(N^2 * M)
// Overall space complexity: O(N^2 * M)
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    if !word_list.contains(&end_word) {
        return 0;
    }

    // SC: N keys, each key could contain up to N entries - O(N^2) 
    let mut graph = HashMap::<&str, Vec<&String>>::new();

    // TCO(N^2 * M), where N is word_list length and M is the max word length
    for word in once(&begin_word).chain(word_list.iter()) {
        let neighbors = find_neighbors(word, &word_list);

        graph.entry(word)
            .or_default()
            .extend(neighbors);
    }

    // SC: O(N * M)
    let mut visited = HashSet::new();
    // SC: O(N * M)
    let mut queue = VecDeque::from([(begin_word.as_str(), 1)]);

    // TC: O(N * M)
    while let Some((word, path_length)) = queue.pop_front() {
        if word == end_word {
            return path_length;
        }

        if !visited.insert(word) {
            continue;
        }

        for n in &graph[word] {
            queue.push_back((n, path_length + 1));
        }
    }

    0
}

// O(N * M), where N is word_list length and M is the max word length
fn find_neighbors<'a>(str: &'a str, word_list: &'a [String]) -> Vec<&'a String> {
    let mut result = vec![];

    // O(N), where N is word_list length
    'outer: for word in word_list.iter() {
        if word.len() != str.len() {
            continue;
        }

        let mut diffs = 0;
        // O(M) where M is the max word length
        for (c1, c2) in str.chars().zip(word.chars()) {
            if c1 != c2 {
                diffs += 1;
            }

            if diffs > 1 {
                continue 'outer;
            }
        }

        if diffs == 1 {
            result.push(word);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()]
            ),
            5
        );
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()]
            ),
            0
        );
    }
}
