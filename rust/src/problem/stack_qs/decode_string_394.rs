// https://leetcode.com/problems/decode-string
pub fn decode_string(string: String) -> String {
    let mut stack: Vec<String> = vec![];

    for char in string.chars() {
        match char {
            ']' => {
                let mut str = String::new();

                while let Some(s) = stack.pop() {
                    if s == "[" {
                        break;
                    }

                    str = s + &str;
                }

                let mut count = String::new();

                while let Some(s) = stack.pop() {
                    if s.chars().all(char::is_numeric) {
                        count = s + &count;
                    } else {
                        stack.push(s);
                        break;
                    }
                }

                str = str.repeat(count.parse::<usize>().unwrap());
                stack.push(str);
            }
            _ => stack.push(char.to_string())
        }
    }

    stack.into_iter().collect()
}

// Key points:
// - two consequitive open brackes is impossible
// - there is always a number before open bracket
// - use shared index to avoid extra job
fn decode_string_rec(s: String) -> String {
    fn solve(s: &str, i: &mut usize) -> String {
        let chars = s.chars().collect::<Vec<_>>();
        let mut result = String::new();

        while *i < chars.len() && chars[*i] != ']' {
            match chars[*i] {
                'a'..='z' => {
                    result.push(chars[*i]);
                    *i += 1;
                }
                // Met a number - parse it and solve sub problem
                _ => {
                    let mut j = *i + 1;
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        j += 1;
                    }
                    let count = s[*i..j].parse::<usize>().unwrap();

                    // j is at the [
                    *i = j + 1; // so skip it

                    let subresult = solve(s, i);
                    result.push_str(&subresult.repeat(count));

                    // Recursive call ended on closing bracket or the s end
                    *i += 1; // so skip it
                }
            }
        }

        result
    }

    solve(&s, &mut 0)
}

#[cfg(test)]
mod tests {
    use crate::problem::stack_qs::decode_string_394::{decode_string, decode_string_rec};

    #[test]
    fn test_decode_string() {
        assert_eq!(decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
        assert_eq!(decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
        assert_eq!(decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
        assert_eq!(decode_string("f3[a2[c]]b".to_string()), "faccaccaccb".to_string());
        assert_eq!(decode_string("10[a]".to_string()), "aaaaaaaaaa".to_string());
        assert_eq!(decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string()), "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string());

        assert_eq!(decode_string_rec("3[a]2[bc]".to_string()), "aaabcbc".to_string());
        assert_eq!(decode_string_rec("3[a2[c]]".to_string()), "accaccacc".to_string());
        assert_eq!(decode_string_rec("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
        assert_eq!(decode_string_rec("f3[a2[c]]b".to_string()), "faccaccaccb".to_string());
        assert_eq!(decode_string_rec("10[a]".to_string()), "aaaaaaaaaa".to_string());
        assert_eq!(decode_string_rec("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string()), "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string());
    }
}