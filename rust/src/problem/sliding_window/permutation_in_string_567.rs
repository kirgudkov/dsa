trait Permutation {
    fn is_permutation(&self, s: &str) -> bool;
}

impl Permutation for String {
    fn is_permutation(&self, s: &str) -> bool {
        let mut counts = std::collections::HashMap::new();

        for c in self.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        for c in s.chars() {
            *counts.entry(c).or_insert(0) -= 1;
        }

        counts.values().all(|&v| v == 0)
    }
}

pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut i = s1.len() - 1;

    while i < s2.len() {
        if s1.is_permutation(&s2[i - (s1.len() - 1)..=i]) {
            return true;
        }
        i += 1;
    }

    false
}

pub fn check_inclusion_2(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let mut s1_count = std::collections::HashMap::new();
    let mut s2_count = std::collections::HashMap::new();

    for i in 0..s1_chars.len() {
        *s1_count.entry(s1_chars[i]).or_insert(0) += 1;
        *s2_count.entry(s2_chars[i]).or_insert(0) += 1;
    }

    for i in s1_chars.len()..s2_chars.len() {
        // HashMap implements PartialEq trait so we can use == operator
        if s1_count == s2_count {
            return true;
        }

        *s2_count.entry(s2_chars[i]).or_insert(0) += 1;

        if let Some(count) = s2_count.get_mut(&s2_chars[i - s1_chars.len()]) {
            *count -= 1;
            if *count == 0 {
                s2_count.remove(&s2_chars[i - s1_chars.len()]);
            }
        }
    }

    s1_count == s2_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1 = "ab";
        let s2 = "eidbaooo";
        assert!(check_inclusion(s1.to_string(), s2.to_string()));
        assert!(check_inclusion_2(s1.to_string(), s2.to_string()));

        let s1 = "ab";
        let s2 = "eidboaoo";
        assert!(!check_inclusion(s1.to_string(), s2.to_string()));
        assert!(!check_inclusion_2(s1.to_string(), s2.to_string()));
    }
}