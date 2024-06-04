struct Solution {
    digits: Vec<u8>,
    mapping: [&'static [char]; 8],
    buf: String,
    result: Vec<String>,
}

impl Solution {
    fn new(digits: String) -> Self {
        Self {
            digits: digits.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
            mapping: [
                &['a', 'b', 'c'],
                &['d', 'e', 'f'],
                &['g', 'h', 'i'],
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

    // Backtracking reimplementation. Still not as efficient as the iterative solution: O(3^N * 4^M)
    fn backtrack(&mut self) {
        if self.digits.is_empty() {
            return;
        }

        if self.buf.len() == self.digits.len() {
            return self.result.push(self.buf.clone());
        }

        // Subtract 2 from the digit to get the index of the corresponding characters: for 2 we should look into mapping[0]
        let digit = self.digits[self.buf.len()] as usize - 2;

        for i in 0..self.mapping[digit].len() {
            self.buf.push(self.mapping[digit][i]);
            self.backtrack();
            self.buf.pop();
        }
    }

    // My first implementation was iterative
    fn solve_iteratively(&self) -> Vec<String> {
        if self.digits.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        let first = &self.mapping[self.digits[0] as usize - 2];

        // fill the result with the first set of characters: "2" -> ["a", "b", "c"]
        for item in first.iter() {
            result.push(item.to_string());
        }

        // Start from the next digit (first item in chars iterator has already been consumed above by .next())
        for c in self.digits.iter().skip(1) {
            // map digit to its corresponding characters: "3" -> ["d", "e", "f"]
            let mapped = &self.mapping[*c as usize - 2];

            // iterate over the result and append each mapped character to each result item
            // 23 -> ["a", "b", "c"] -> ["d", "e", "f"] => ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            let mut temp = Vec::new();
            for i in result {
                for j in mapped.iter() {
                    temp.push(format!("{}{}", i, j));
                }
            }

            // overwrite the result with the new combinations
            result = temp;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::letter_combinations_of_a_phone_number::Solution;

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
