pub fn reverse_string(s: &mut [char]) {
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        s.swap(l, r);

        l += 1;
        r -= 1;
    }
}


// alternative recursive approach: reverse(s, 0, s.len() - 1);
fn reverse(s: &mut Vec<char>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    s.swap(start, end);
    reverse(s, start + 1, end - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}