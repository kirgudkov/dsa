// https://leetcode.com/problems/roman-to-integer
// TC: O(n), SC: O(1)
pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;

    let dict = std::collections::HashMap::from([
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000),
    ]);


    let mut i = 0;
    let chars = s.chars().collect::<Vec<char>>();

    while i < s.len() {
        let curr = *dict.get(&chars[i]).unwrap();

        if i == s.len() - 1 {
            result += curr;
            break;
        }

        let next = *dict.get(&chars[i + 1]).unwrap();

        if curr >= next {
            result += curr;
            i += 1;
        } else {
            result += next - curr;
            i += 2;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}