pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Bt::solve(nums)
}

#[derive(Default)]
struct Bt {
    result: Vec<Vec<i32>>,
    nums: Vec<i32>,
    buf: Vec<i32>,
}

impl Bt {
    pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut instance = Bt { nums, ..Default::default() };
        instance.backtrack();
        instance.result
    }

    fn backtrack(&mut self) {
        if self.buf.len() == self.nums.len() {
            return self.result.push(self.buf.clone());
        }

        for i in 0..self.nums.len() {
            if self.buf.contains(&self.nums[i]) {
                continue;
            }

            self.buf.push(self.nums[i]);
            self.backtrack();
            self.buf.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let result = permute(nums);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }
}