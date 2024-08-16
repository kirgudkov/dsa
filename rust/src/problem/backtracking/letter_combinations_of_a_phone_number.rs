struct Solution {
    digits: Vec<u8>,
    numpad: [&'static [char]; 8],
    buf: String,
    result: Vec<String>,
}

impl Solution {
    fn new(digits: String) -> Self {
        Self {
            digits: digits.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),

            numpad: [
                &['a', 'b', 'c'], // for button with digit 2
                &['d', 'e', 'f'], // for 3
                &['g', 'h', 'i'], // etc ...
                &['j', 'k', 'l'],
                &['m', 'n', 'o'],
                &['p', 'q', 'r', 's'],
                &['t', 'u', 'v'],
                &['w', 'x', 'y', 'z'],
            ],
            buf: String::new(),
            result: Vec::new(),
        }
    }

    // TC is O(4^n) in the worst case scenario, where input has all 7 or/and 9 (both have 4 chars in mapping)
    fn backtrack(&mut self) {
        if self.digits.is_empty() {
            return;
        }

        if self.buf.len() == self.digits.len() {
            return self.result.push(self.buf.clone());
        }

        // digits[buf.len()] is a trick to get current digit without passign current position into backtrack():
        // e.g. when buf.len() is 0, we're working on the first digit in input
        let digit = self.digits[self.buf.len()]
            // Subtract 2 from the digit to get the index of the corresponding characters: for 2 we should look into mapping[0]
            as usize - 2;

        for &ch in self.numpad[digit] {
            self.buf.push(ch);
            self.backtrack();
            self.buf.pop();
        }
    }

    // TC is the same - O(4^n)
    fn solve_iteratively(&self) -> Vec<String> {
        if self.digits.is_empty() {
            return vec![];
        }

        let mut res = vec![];

        // fill the result with the first set of characters: "2" -> ["a", "b", "c"]
        for ch in self.numpad[self.digits[0] as usize - 2] {
            res.push(ch.to_string());
        }

        // Start from the next digit
        for digit in self.digits.iter().skip(1) {
            // iterate over the result and append each mapped character to each result item
            // 23 -> ["a", "b", "c"] -> ["d", "e", "f"] => ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            let mut new_res = Vec::new();

            for prev_str in res {
                for next_char in self.numpad[*digit as usize - 2].iter() {
                    new_res.push(format!("{}{}", prev_str, next_char));
                }
            }

            res = new_res;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let mut ctx = Solution::new("23".to_string());
        assert_eq!(
            ctx.solve_iteratively(),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        ctx.backtrack();
        assert_eq!(
            ctx.result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );

        let mut ctx = Solution::new("234".to_string());
        assert_eq!(
            ctx.solve_iteratively(),
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi", "beg", "beh",
                "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi",
            ]
        );
        ctx.backtrack();
        assert_eq!(
            ctx.result,
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi", "beg", "beh",
                "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi",
            ]
        );

        let mut ctx = Solution::new("".to_string());
        assert_eq!(ctx.solve_iteratively(), Vec::<String>::new());
        ctx.backtrack();
        assert_eq!(ctx.result, Vec::<String>::new());

        let mut ctx = Solution::new("2".to_string());
        assert_eq!(ctx.solve_iteratively(), vec!["a", "b", "c"]);
        ctx.backtrack();
        assert_eq!(ctx.result, vec!["a", "b", "c"]);
    }
}
