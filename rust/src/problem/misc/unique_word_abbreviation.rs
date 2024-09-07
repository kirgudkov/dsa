use std::collections::{HashMap, HashSet};

struct ValidWordAbbr {
    map: HashMap<String, HashSet<String>>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut map = HashMap::new();

        for word in dictionary {
            map.entry(Self::get_abr(&word))
                .or_insert(HashSet::new())
                .insert(word);
        }

        Self { map }
    }

    fn is_unique(&self, word: String) -> bool {
        let key = Self::get_abr(&word);

        if let Some(set) = self.map.get(&key) {
            return set.len() == 1 && set.contains(&word);
        }

        true
    }

    fn get_abr(word: &str) -> String {
        if word.len() < 3 {
            return word.to_string();
        }

        format!("{}{}{}", word.chars().nth(0).unwrap(), word.len().saturating_sub(2), word.chars().last().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vwa = ValidWordAbbr::new(
            vec![
                "deer".to_string(), "door".to_string(),
                "cake".to_string(), "card".to_string(),
                "f".to_string(), "rr".to_string(),  "rrr".to_string(),
            ]
        );
        assert!(!vwa.is_unique("dear".to_string()));
        assert!(!vwa.is_unique("door".to_string()));
        assert!(vwa.is_unique("cart".to_string()));
        assert!(!vwa.is_unique("cane".to_string()));
        assert!(vwa.is_unique("make".to_string()));
        assert!(vwa.is_unique("cake".to_string()));
        assert!(vwa.is_unique("f".to_string()));
        assert!(vwa.is_unique("a".to_string()));
        assert!(vwa.is_unique("r".to_string()));
        assert!(vwa.is_unique("rr".to_string()));
        assert!(vwa.is_unique("rrr".to_string()));
        assert!(!vwa.is_unique("rer".to_string()));
    }
}