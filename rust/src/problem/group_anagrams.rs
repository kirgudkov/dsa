// https://leetcode.com/problems/group-anagrams
pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    for string in &strings {
        let mut key = string.chars().collect::<Vec<char>>();
        key.sort();
        let entry = map.entry(key).or_insert(vec![]);
        entry.push(string.clone());
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let mut expect: Vec<Vec<String>> = vec![
            ["eat", "tea", "ate"].iter().map(|s| s.to_string()).collect(),
            ["tan", "nat"].iter().map(|s| s.to_string()).collect(),
            ["bat"].iter().map(|s| s.to_string()).collect(),
        ];
        expect.sort();

        let mut result = group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect());

        result.sort();

        assert_eq!(result, expect);
    }
}