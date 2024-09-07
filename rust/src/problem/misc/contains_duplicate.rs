pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hs = std::collections::HashSet::new();

    for num in nums {
        if hs.contains(&num) {
            return true;
        }

        hs.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}