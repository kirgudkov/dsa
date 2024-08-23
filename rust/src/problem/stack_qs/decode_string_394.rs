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

#[cfg(test)]
mod tests {
    use crate::problem::stack_qs::decode_string_394::decode_string;

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