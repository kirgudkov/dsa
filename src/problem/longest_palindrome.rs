pub fn longest_palindrome(s: String) -> String {
    let chars = s.as_bytes();
    let mut result = (0, 0);

    for i in 0..chars.len() * 2 {
        let mut l = i / 2;
        let mut r = i / 2 + if i % 2 == 0 { 0 } else { 1 };

        while l <= r && l < chars.len() && r < chars.len() {
            if chars[l] == chars[r] {
                if r - l > result.1 - result.0 {
                    result = (l, r)
                }
            } else {
                break;
            }

            if l > 0 {
                l -= 1;
            } else {
                break;
            };

            r += 1;
        }
    }

    s[result.0..=result.1].to_string()
}

// println!("Longest palindrome");
// println!("Longest of \"cbbd\" is: {}", crate::longest_palindrome::longest_palindrome(String::from("cbbd")));
// println!("Longest of \"alevelevelqweytty\" is: {}", crate::longest_palindrome::longest_palindrome(String::from("alevelevelqweytty")));
// println!("Longest of \"aaabaaaa\" is: {}", crate::longest_palindrome::longest_palindrome(String::from("aaabaaaa")));
