pub fn generate_parenthesis(n: i32) -> Vec<String> {
    Bt::new(n).result
}

#[derive(Default)]
struct Bt {
    n: usize,
    buf: String,
    result: Vec<String>,
}

impl Bt {
    fn new(n: i32) -> Self {
        let mut instance = Bt { n: n as usize, ..Default::default() };
        instance.backtrack();
        instance
    }

    fn backtrack(&mut self) {
        if self.buf.len() == self.n * 2 {
            return self.result.push(self.buf.clone());
        }

        for p in ['(', ')'] {
            if self.is_valid(p) {
                self.buf.push(p);
                self.backtrack();
                self.buf.pop();
            }
        }
    }

    // 1. We can insert '(' only if the number of '(' is less than n; - "There is space for an open parenthesis"
    // 2. We can insert ')' only if the number of ')' is less than the number of '('; - "There is an open parenthesis to be closed"
    // e.g, we cannot insert ')' at 0 position, because there is no open parenthesis to close yet. 
    fn is_valid(&self, c: char) -> bool {
        let open_count = self.buf.chars().filter(|&c| c == '(').count();
        let close_count = self.buf.chars().filter(|&c| c == ')').count();

        if c == '(' {
            open_count < self.n
        } else {
            close_count < open_count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let n = 3;
        let result = generate_parenthesis(n);
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(result, expected);
    }
}
