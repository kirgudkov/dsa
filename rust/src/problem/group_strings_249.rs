use std::collections::HashMap;

// https://leetcode.com/problems/group-shifted-strings
pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for string in strings {
        let key = {
            // We care only about right shifts, because it should be consistent:
            // we're not calculating "distance" because distance would be the same in both directions.
            // In our case, for "ab" we'd have [1] and for "ba" it will be [25];
            // This would sort those two in different groups even though the "distance" is 1
            let r_shifts: Vec<i8> = string.as_bytes().windows(2)
                .map(|w| (w[1] as i8 - w[0] as i8 + 26) % 26)
                .collect();
            // Each group is defined by the string length and the shifts pattern;
            // For example, all strings with the size of 3 and shifts [1, 1] are in the same group: "abc", "bcd", "xyz", etc.
            // becuase you can transform one into another by shifting each char n times to the right.
            (string.len(), r_shifts)
        };

        map.entry(key).or_insert(vec![]).push(string);
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = group_strings(vec!["abc".to_string(), "bcd".to_string(), "acef".to_string(), "xyz".to_string(), "az".to_string(), "ba".to_string(), "a".to_string(), "z".to_string()]);
        result.sort();

        let mut expected = vec![
            vec!["a".to_string(), "z".to_string()],
            vec!["abc".to_string(), "bcd".to_string(), "xyz".to_string()],
            vec!["acef".to_string()],
            vec!["az".to_string(), "ba".to_string()],
        ];
        expected.sort();

        assert_eq!(result, expected);
    }
}