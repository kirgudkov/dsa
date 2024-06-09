// https://leetcode.com/problems/isomorphic-strings
pub fn is_isomorphic(s: String, t: String) -> bool {
    // Originally, a HashMaps were used to store the mappings.
    let mut s_to_t: Vec<Option<u8>> = vec![None; 256];
    let mut t_to_s: Vec<Option<u8>> = vec![None; 256];

    s.bytes()
        .zip(t.bytes())
        .all(|(sc, tc)| {
            match (s_to_t[sc as usize], t_to_s[tc as usize]) {
                // Both characters already have mappings: Ensure they are consistent with the current pair:
                (Some(mapped_tc), Some(mapped_sc)) => mapped_tc == tc && mapped_sc == sc,
                // None of the characters have mappings: Create new mappings:
                (None, None) => {
                    s_to_t[sc as usize] = Some(tc);
                    t_to_s[tc as usize] = Some(sc);
                    true
                }
                // One of the characters has a mapping while the other doesn't: 
                // Inconsistent mappings. Return false.
                _ => false,
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