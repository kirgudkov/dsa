use std::collections::{HashMap, HashSet, VecDeque};

// https://leetcode.com/problems/alien-dictionary/
pub fn alien_order(words: Vec<String>) -> String {
    if words.len() < 2 {
        return words[0].chars().collect::<HashSet<char>>().into_iter().collect();
    }

    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    let mut indegrees: HashMap<char, usize> = HashMap::new();

    // 0. Convert all words into disconnected graph (each letter is a vertex with no neighbours and 0 in-degree).
    words.iter().for_each(|word| {
        word.chars().for_each(|c| {
            graph.entry(c).or_default();
            *indegrees.entry(c).or_default() += 0;
        });
    });

    // 1. Traverse words in pairs and find order relationships between letters (if there are).
    for window in words.windows(2) {
        for (a, b) in window[0].chars().zip(window[1].chars()) {
            if a != b { // As soon as we've found two distinct chars, we have discovered a relationship between these two.
                graph.entry(a).or_default().push(b); // Now we know that b comes after a 
                *indegrees.entry(b).or_default() += 1; // and vertex b got one more in-degree
                break;
            }
        }

        // This is not obvious, but [ab abc] is sorted whereas [abc ab] is not sorted
        if window[0].len() > window[1].len() && window[0].starts_with(&window[1]) {
            return "".to_string();
        }
    }

    // 2. Apply Kahn's topological sort
    let mut queue: VecDeque<char> = indegrees.iter()
        .filter_map(|(&c, &degree)| {
            (degree == 0).then_some(c)
        })
        .collect();

    let mut alphabet: Vec<char> = vec![];

    while let Some(letter) = queue.pop_front() {
        alphabet.push(letter);

        for neighbor in graph.get(&letter).unwrap() {
            let indegree = indegrees.get_mut(neighbor).unwrap();
            *indegree -= 1;

            if *indegree == 0 {
                queue.push_back(*neighbor);
            }
        }
    }

    if alphabet.len() == graph.len() {
        alphabet.into_iter().collect()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(), "ett".to_string(), "rftt".to_string()];
        assert_eq!(alien_order(words), "wertf".to_string());

        let words = vec!["z".to_string(), "x".to_string()];
        assert_eq!(alien_order(words), "zx".to_string());

        let words = vec!["z".to_string(), "x".to_string(), "z".to_string()];
        assert_eq!(alien_order(words), "".to_string());

        let words = vec!["b".to_string()];
        assert_eq!(alien_order(words), "b".to_string());

        let words = vec!["z".to_string(), "z".to_string()];
        assert_eq!(alien_order(words), "z".to_string());

        let words = vec!["abc".to_string(), "ab".to_string()];
        assert_eq!(alien_order(words), "".to_string());
    }
}