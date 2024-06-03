pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ctx = Ctx::new(n as usize, k as usize);
    ctx.backtrack(1);
    ctx.result
}

struct Ctx {
    n: usize,
    k: usize,
    buf: Vec<i32>,
    result: Vec<Vec<i32>>,
}

impl Ctx {
    fn new(n: usize, k: usize) -> Self {
        Self {
            n,
            k,
            buf: vec![],
            result: vec![],
        }
    }

    fn backtrack(&mut self, start: usize) {
        if self.buf.len() == self.k {
            self.result.push(self.buf.clone());
            return;
        }

        for cur in start..=self.n {
            self.buf.push(cur as i32);
            self.backtrack(cur + 1);
            self.buf.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::combine::combine;

    #[test]
    fn test_1() {
        assert_eq!(combine(4, 2), vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
    }
}
