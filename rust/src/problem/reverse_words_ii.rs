// https://leetcode.com/problems/reverse-words-in-a-string-ii
// TC: O(N), SC: O(1)
pub fn reverse_words(s: &mut [char]) {
    let mut l = 0;
    let mut r = s.len() - 1;

    // Reverse the entire string
    while l < r {
        s.swap(l, r);

        l += 1;
        r -= 1;
    }

    // Reverse each word
    l = 0;
    r = 0;

    while r < s.len() {
        while r < s.len() && s[r] != ' ' {
            r += 1;
        }

        let mut _r = r - 1;
        let mut _l = l;

        while _l < _r {
            s.swap(_l, _r);
            _r -= 1;
            _l += 1;
        }

        r += 1;
        l = r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let mut s = vec!['t', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e'];
        reverse_words(&mut s);
        assert_eq!(s, vec!['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e']);

        let mut s = vec!['a'];
        reverse_words(&mut s);
        assert_eq!(s, vec!['a']);
    }
}