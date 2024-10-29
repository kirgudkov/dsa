fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.iter().fold(strs.first().unwrap_or(&String::new()).clone(), |acc, s| {
        acc.chars()
            .zip(s.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let result = longest_common_prefix(strs);
        assert_eq!(result, "fl");
    }
}
