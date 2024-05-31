pub fn decode_string(s: String) -> String {
    let mut stack = Vec::new();
    stack.push("".to_string());

    for char in s.chars() {
        match char {
            '[' => stack.push("".to_string()),
            ']' => {
                let right = stack.pop().unwrap();
                let count = stack.pop().unwrap().parse::<usize>().unwrap();
                let left = stack.pop().unwrap();

                stack.push(left + right.repeat(count).as_str());
            }
            char => {
                if let Some(last) = stack.last_mut() {
                    if char.is_ascii_digit() {
                        if last.parse::<usize>().is_ok() {
                            *last += &char.to_string();
                        } else {
                            stack.push(char.to_string());
                        }
                    } else {
                        *last += &char.to_string();
                    }
                } else {
                    stack.push(char.to_string());
                }
            }
        }
    }

    stack.join("")
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