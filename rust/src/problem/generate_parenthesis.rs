pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ctx = Ctx::new(n as usize);
    ctx.backtrack();
    ctx.result
}

struct Ctx {
    n: usize,
    buf: String,
    result: Vec<String>,
}

impl Ctx {
    fn new(n: usize) -> Self {
        Self {
            n,
            buf: String::new(),
            result: vec![],
        }
    }

    fn backtrack(&mut self) {
        if self.buf.len() == self.n * 2 {
            self.result.push(self.buf.clone());
            return;
        }

        for parenthesis in &["(", ")"] {
            if !self.is_valid(parenthesis) {
                continue;
            }

            self.buf.push_str(parenthesis);
            self.backtrack();
            self.buf.pop();
        }
    }

    // This fn magically drops all the invalid entries:
    // 1. We can insert an open parenthesis only if the number of open parenthesis is less than n; - "There is space for an open parenthesis"
    // 2. We can insert a close parenthesis only if the number of close parenthesis is less than the number of open parenthesis; - "There is an open parenthesis to close"
    // So, for example, we cannot insert ")" at first position, because there is no open parenthesis to close. 
    fn is_valid(&self, sym: &str) -> bool {
        let open = self.buf.chars().filter(|&c| c == '(').count();
        let close = self.buf.chars().filter(|&c| c == ')').count();

        if sym == "(" {
            open < self.n
        } else {
            close < open
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
