use std::collections::HashSet;

pub fn total_n_queens(n: i32) -> i32 {
    backtrack(0, n, &mut Ctx::new()).count
}

fn backtrack(i: i32, n: i32, ctx: &mut Ctx) -> &mut Ctx {
    if i == n {
        ctx.count += 1;
    }

    for j in 0..n {
        if !ctx.is_safe(i, j) {
            continue;
        }

        ctx.place(i, j);
        backtrack(i + 1, n, ctx);
        ctx.remove(i, j);
    }

    ctx
}

struct Ctx {
    count: i32,
    cols: HashSet<i32>,
    diags_1: HashSet<i32>,
    diags_2: HashSet<i32>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            count: 0,
            cols: HashSet::new(),
            diags_1: HashSet::new(),
            diags_2: HashSet::new(),
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
