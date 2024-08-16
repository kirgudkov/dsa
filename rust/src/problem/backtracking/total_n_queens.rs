use std::collections::HashSet;

pub fn total_n_queens(n: i32) -> i32 {
    Backtracking::solve(n)
}

#[derive(Default)]
struct Backtracking {
    count: i32,
    cols: HashSet<i32>,
    diags_1: HashSet<i32>,
    diags_2: HashSet<i32>,
}

impl Backtracking {
    pub fn solve(n: i32) -> i32 {
        let mut bt = Backtracking::default();
        bt.backtrack(n, 0);
        bt.count
    }

    fn backtrack(&mut self, n: i32, i: i32) {
        if i == n {
            self.count += 1;
        }

        for j in 0..n {
            if !self.is_safe(i, j) {
                continue;
            }

            self.place(i, j);
            self.backtrack(n, i + 1);
            self.remove(i, j);
        }
    }

    fn place(&mut self, i: i32, j: i32) {
        self.cols.insert(j);
        self.diags_1.insert(i - j);
        self.diags_2.insert(i + j);
    }

    fn remove(&mut self, i: i32, j: i32) {
        self.cols.remove(&j);
        self.diags_1.remove(&(i - j));
        self.diags_2.remove(&(i + j));
    }

    fn is_safe(&self, i: i32, j: i32) -> bool {
        !self.cols.contains(&j)
            && !self.diags_1.contains(&(i - j))
            && !self.diags_2.contains(&(i + j))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(total_n_queens(4), 2);
        assert_eq!(total_n_queens(1), 1);
    }
}
