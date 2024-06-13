use std::collections::HashSet;

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut ctx = Ctx::new(n);
    ctx.backtrack(0);
    ctx.boards
}

fn backtrack(i: i32, n: i32, ctx: &mut Ctx) {
    if i == n {
        ctx.boards.push(ctx.board.clone());
    }

    for j in 0..n {
        if !ctx.is_under_attack(i, j) {
            continue;
        }

        ctx.place(i, j);
        backtrack(i + 1, n, ctx);
        ctx.remove(i, j);
    }
}

struct Ctx {
    board: Vec<String>,
    boards: Vec<Vec<String>>,
    cols: HashSet<i32>,
    diag_1: HashSet<i32>,
    diag_2: HashSet<i32>,
}

impl Ctx {
    fn new(n: i32) -> Self {
        Self {
            board: vec![".".repeat(n as usize); n as usize],
            boards: vec![],
            cols: HashSet::new(),
            diag_1: HashSet::new(),
            diag_2: HashSet::new(),
        }
    }

    fn place(&mut self, i: i32, j: i32) {
        self.cols.insert(j);
        self.diag_1.insert(i - j);
        self.diag_2.insert(i + j);

        let row = &mut self.board[i as usize];
        row.replace_range(j as usize..=j as usize, "Q");
    }

    fn remove(&mut self, i: i32, j: i32) {
        self.cols.remove(&j);

        self.diag_1.remove(&(i - j));
        self.diag_2.remove(&(i + j));

        let row = &mut self.board[i as usize];
        row.replace_range(j as usize..=j as usize, ".");
    }

    fn is_under_attack(&self, i: i32, j: i32) -> bool {
        self.cols.contains(&j)
            || self.diag_1.contains(&(i - j))
            || self.diag_2.contains(&(i + j))
    }

    fn backtrack(&mut self, i: i32) {
        if i == self.board.len() as i32 {
            self.boards.push(self.board.clone());
        }

        for j in 0..self.board.len() as i32 {
            if self.is_under_attack(i, j) {
                continue;
            }

            self.place(i, j);
            self.backtrack(i + 1);
            self.remove(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let res = vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."],
        ];
        assert_eq!(solve_n_queens(n), res);
    }
}
