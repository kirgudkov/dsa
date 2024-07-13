pub fn h_index_ii(citations: Vec<i32>) -> i32 {
    let n = citations.len() as i32;
    let mut i = 0;

    for c in citations {
        if c >= n - i {
            return n - i;
        } else {
            i += 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_index_ii() {
        assert_eq!(h_index_ii(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(h_index_ii(vec![0, 1, 4, 5, 6, 7]), 4);
        assert_eq!(h_index_ii(vec![0, 1, 4, 5, 6, 7, 8]), 4);
        assert_eq!(h_index_ii(vec![0, 1, 4, 5, 6]), 3);
        assert_eq!(h_index_ii(vec![0, 1, 4, 5, 6, 7, 8, 9]), 5);
        assert_eq!(h_index_ii(vec![0, 1, 4, 5, 6, 7, 8, 9, 10]), 5);
    }
}