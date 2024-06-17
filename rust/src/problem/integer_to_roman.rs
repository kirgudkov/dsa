use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let mut result = String::new();

    let ones_vocab = HashMap::from([(1, "I"), (5, "V"), (10, "X")]);
    let tens_vocab = HashMap::from([(1, "X"), (5, "L"), (10, "C")]);
    let hundreds_vocab = HashMap::from([(1, "C"), (5, "D"), (10, "M")]);
    let thousands_vocab = HashMap::from([(1, "M")]);

    let ones = num % 10;
    let tens = num % 100 / 10;
    let hundreds = num % 1000 / 100;
    let thousands = num / 1000;

    for _ in 0..thousands {
        result.push_str(thousands_vocab[&1]);
    }

    result.push_str(&digit_to_roman(hundreds, &hundreds_vocab));
    result.push_str(&digit_to_roman(tens, &tens_vocab));
    result.push_str(&digit_to_roman(ones, &ones_vocab));

    result
}

fn digit_to_roman(digit: i32, vocab: &HashMap<i32, &str>) -> String {
    let mut roman = String::new();

    match digit {
        9 => roman.push_str((String::new() + vocab[&1] + vocab[&10]).as_str()),
        4 => roman.push_str((String::new() + vocab[&1] + vocab[&5]).as_str()),
        _ => {
            if digit >= 5 {
                roman.push_str(vocab[&5]);
            }
            for _ in 0..digit % 5 {
                roman.push_str(vocab[&1]);
            }
        }
    }

    roman
}

#[cfg(test)]
mod tests {
    use crate::problem::integer_to_roman::int_to_roman;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
