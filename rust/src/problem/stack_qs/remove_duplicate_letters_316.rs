use std::collections::{HashMap, HashSet};

pub fn remove_duplicate_letters(s: String) -> String {
    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    let mut last_i = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        last_i.insert(c, i);
    }

    for (i, current) in s.chars().enumerate() {
        if seen.insert(current) {
            while let Some(&recent) = stack.last() {
                if current < recent && i < last_i[&recent] {
                    seen.remove(&stack.pop().unwrap());
                } else {
                    break;
                }
            }

            stack.push(current);
        }
    }

    stack.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "ecbacba".to_string();
        assert_eq!(remove_duplicate_letters(s), "eacb".to_string());

        let s = "bcabc".to_string();
        assert_eq!(remove_duplicate_letters(s), "abc".to_string());

        let s = "cbacdcbc".to_string();
        assert_eq!(remove_duplicate_letters(s), "acdb".to_string());
    }
}
