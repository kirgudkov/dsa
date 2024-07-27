pub fn add_binary(a: String, b: String) -> String {
    let mut res = String::new();
    let mut carry = 0;

    let mut chars_a = a.chars().rev();
    let mut chars_b = b.chars().rev();

    loop {
        match (chars_a.next(), chars_b.next()) {
            (None, None) => {
                if carry > 0 {
                    res.push('1');
                }

                break;
            }
            (a, b) => {
                let sum = carry
                    + a.unwrap_or('0').to_digit(2).unwrap()
                    + b.unwrap_or('0').to_digit(2).unwrap();

                res.push_str(&format!("{}", sum % 2));
                carry = sum / 2;
            }
        }
    }

    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100".to_string());
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
        assert_eq!(add_binary("0".to_string(), "0".to_string()), "0".to_string());
    }
}