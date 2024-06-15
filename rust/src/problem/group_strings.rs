use std::collections::HashMap;

pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<(usize, Vec<i8>), Vec<String>> = HashMap::new();

    for string in strings {
        let key = if string.len() > 1 {
            let mut distances = vec![];
            for i in 1..string.len() {
                // treat bytes as signed integers to allow overflow
                distances.push((string.as_bytes()[i] as i8 - string.as_bytes()[i - 1] as i8 + 26) % 26);
            }

            (string.len(), distances)
        } else {
            (string.len(), vec![])
        };

        let entry = map.entry(key).or_default();
        entry.push(string);
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