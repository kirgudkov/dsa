pub fn is_palindrome(s: String) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;
    let s = s.chars().collect::<Vec<_>>();

    while l < r {
        if !matches!(s[l].to_ascii_lowercase(), '0'..='9' | 'a'..='z') {
            l += 1;
            continue;
        }

        if !matches!(s[r].to_ascii_lowercase(), '0'..='9' | 'a'..='z') {
            r -= 1;
            continue;
        }

        if s[l].to_ascii_lowercase() != s[r].to_ascii_lowercase() {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

pub fn is_palindrome_iter(s: String) -> bool {
    let filtered = s.chars().filter_map(|c| {
        c.is_alphanumeric().then_some(c.to_ascii_lowercase())
    }).collect::<String>();

    filtered.chars().eq(filtered.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert!(is_palindrome_iter("A man, a plan, a canal: Panama".to_string()));
    }
}