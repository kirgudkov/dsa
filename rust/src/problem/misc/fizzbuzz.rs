pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n).map(|i| match (i % 3, i % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => i.to_string()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fizz_buzz(15), vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string(),
        ]);
    }
}