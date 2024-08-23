// https://leetcode.com/problems/longest-repeating-character-replacement/
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut freq = [0usize; 27]; // last item to hold max freq
    let mut result = i32::MIN;
    let bytes = s.as_bytes();

    let index = |byte: &u8| -> usize {
        (byte - b'A') as usize // b'A' -> 65u8;
    };

    let mut l = 0;

    for (r, byte) in bytes.iter().enumerate() {
        freq[index(byte)] += 1;
        freq[26] = freq[26].max(freq[index(byte)]);

        if r - l + 1 - freq[26] > k as usize {
            freq[index(&bytes[l])] -= 1;
            l += 1;
        }

        result = result.max((r - l + 1) as i32);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}