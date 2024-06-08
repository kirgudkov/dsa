pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut l = 0;
    let mut r = letters.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if letters[m as usize] > target {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    if l < letters.len() as i32 {
        letters[l as usize]
    } else {
        letters[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'g'), 'j');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'j'), 'c');
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'k'), 'c');

        assert_eq!(next_greatest_letter(vec!['a', 'b'], 'z'), 'a');

        assert_eq!(next_greatest_letter(vec!['a', 'b', 'b', 'b', 'd'], 'b'), 'd');
        assert_eq!(next_greatest_letter(vec!['a', 'b', 'b', 'b', 'd'], 'f'), 'a');
    }
}