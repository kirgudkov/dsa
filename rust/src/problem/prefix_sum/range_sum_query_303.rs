struct NumArray {
    prefixes: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefixes = vec![0; nums.len()];
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            prefixes[i] = sum;
        }
        Self { prefixes }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.prefixes[right as usize]
        } else {
            self.prefixes[right as usize] - self.prefixes[left as usize - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}