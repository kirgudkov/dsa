// https://leetcode.com/problems/isomorphic-strings
pub fn is_isomorphic(a: String, b: String) -> bool {
    // Originally, a HashMaps were used to store the mappings.
    let mut a_to_b: Vec<Option<u8>> = vec![None; 256];
    let mut b_to_a: Vec<Option<u8>> = vec![None; 256];

    a.bytes()
        .zip(b.bytes())
        .all(|(a_char, b_char)| {
            match (a_to_b[a_char as usize], b_to_a[b_char as usize]) {
                // Both characters have mappings: Ensure they are consistent with the current pair:
                (Some(mapped_b), Some(mapped_a)) => mapped_b == b_char && mapped_a == a_char,
                // None of the characters have mappings: Create new mappings:
                (None, None) => {
                    a_to_b[a_char as usize] = Some(b_char);
                    b_to_a[b_char as usize] = Some(a_char);
                    true
                }
                // One of the characters has a mapping while the other doesn't: 
                // Inconsistent mappings. Return false.
                _ => false
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
        assert!(!is_isomorphic("badc".to_string(), "baba".to_string()));
    }
}