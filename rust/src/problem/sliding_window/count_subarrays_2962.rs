// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max = nums.iter().max().unwrap();

    let mut count = 0;
    let mut result = 0;
    let mut l = 0;

    for num in &nums {
        if num == max {
            count += 1;
        }

        while count == k {
            if nums[l] == *max { count -= 1 }
            l += 1;
        }

        result += l;
    }

    result as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3, 3], 2), 11);
        assert_eq!(count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
}
