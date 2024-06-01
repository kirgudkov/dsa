// https://leetcode.com/problems/decode-string
pub fn decode_string(s: String) -> String {
    let mut stack: std::collections::VecDeque<String> = std::collections::VecDeque::new();

    for char in s.chars() {
        match char {
            ']' => {
                let mut str = String::new();
                while let Some(last) = stack.pop_back() {
                    if last == "[" {
                        break;
                    }

                    str = last + &str;
                }

                let mut count = String::new();
                while let Some(last) = stack.pop_back() {
                    if last.chars().all(char::is_numeric) {
                        count = last + &count;
                    } else {
                        stack.push_back(last);
                        break;
                    }
                }

                stack.push_back(str.repeat(count.parse::<usize>().unwrap()));
            }
            _ => stack.push_back(char.to_string())
        }
    }

    stack.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::problem::decode_string::decode_string;

    #[test]
    fn test_decode_string() {
        assert_eq!(decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
        assert_eq!(decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
        assert_eq!(decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
        assert_eq!(decode_string("f3[a2[c]]b".to_string()), "faccaccaccb".to_string());
        assert_eq!(decode_string("10[a]".to_string()), "aaaaaaaaaa".to_string());
        assert_eq!(decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string()), "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string());
    }
}