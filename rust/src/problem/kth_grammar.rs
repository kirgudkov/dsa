pub fn kth_grammar(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    let nth_row_length = 2i32.pow((n - 1) as u32);

    if k > nth_row_length / 2 {
        1 - kth_grammar(n - 1, k - (nth_row_length / 2))
    } else {
        kth_grammar(n - 1, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_grammar() {
        assert_eq!(kth_grammar(1, 1), 0);
        assert_eq!(kth_grammar(2, 1), 0);
        assert_eq!(kth_grammar(2, 2), 1);
        assert_eq!(kth_grammar(4, 5), 1);
        assert_eq!(kth_grammar(30, 434991989), 0);
    }
}
