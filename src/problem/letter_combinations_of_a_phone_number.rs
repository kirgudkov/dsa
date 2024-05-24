pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    let mapping = vec![
        vec![], vec![], vec!["a", "b", "c"], vec!["d", "e", "f"],
        vec!["g", "h", "i"], vec!["j", "k", "l"], vec!["m", "n", "o"],
        vec!["p", "q", "r", "s"], vec!["t", "u", "v"], vec!["w", "x", "y", "z"],
    ];

    let mut chars = digits.chars();
    let first = &mapping[chars.next().unwrap().to_digit(10).unwrap() as usize];

    // fill the result with the first set of characters: "2" -> ["a", "b", "c"]
    for item in first {
        result.push(item.to_string());
    }

    // Start from the next digit (first item in chars iterator has already been consumed above by .next())
    for c in chars {
        // map digit to its corresponding characters: "3" -> ["d", "e", "f"]
        let mapped = &mapping[c.to_digit(10).unwrap() as usize];

        // iterate over the result and append each mapped character to each result item
        // 23 -> ["a", "b", "c"] -> ["d", "e", "f"] => ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        let mut temp = Vec::new();
        for i in result {
            for j in mapped {
                temp.push(format!("{}{}", i, j));
            }
        }

        // overwrite the result with the new combinations
        result = temp;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::letter_combinations_of_a_phone_number::letter_combinations;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(letter_combinations(String::from("23")), vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        assert_eq!(letter_combinations(String::from("234")), vec!["adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi"]);
        assert_eq!(letter_combinations(String::from("")), vec![] as Vec<String>);
        assert_eq!(letter_combinations(String::from("2")), vec!["a", "b", "c"]);
    }
}
