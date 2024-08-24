// https://leetcode.com/problems/target-sum/
// TC is O(2^n) and SC is O(n);
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut stack = vec![(0, 0)];
    let mut count = 0;

    while let Some((pos, sum)) = stack.pop() {
        if pos != nums.len() {
            stack.push((pos + 1, sum + nums[pos]));
            stack.push((pos + 1, sum - nums[pos]));
        } else if sum == target {
            count += 1;
        }
    }

    count
}

fn find_target_sum_ways_rec(nums: Vec<i32>, target: i32) -> i32 {
    let total = nums.iter().sum::<i32>();
    let mut memo = vec![vec![None; (total as usize * 2) + 1]; nums.len()];

    fn count(nums: &[i32], total: i32, i: usize, sum: i32, target: i32, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if i == nums.len() {
            if sum == target { 1 } else { 0 }
        } else if let Some(s) = memo[i][(sum + total) as usize] {
            s
        } else {
            let a = count(nums, total, i + 1, sum + nums[i], target, memo);
            let b = count(nums, total, i + 1, sum - nums[i], target, memo);
            memo[i][(sum + total) as usize] = Some(a + b);
            a + b
        }
    }

    count(&nums, total, 0, 0, target, &mut memo)
}

// There is a better approach using DP with TC O(n * sum) and SC O(n * sum);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
        assert_eq!(find_target_sum_ways(vec![2, 1], 1), 1);
        assert_eq!(find_target_sum_ways(vec![2, 1], 0), 0);
    }

    #[test]
    fn test_find_target_sum_ways_rec() {
        assert_eq!(find_target_sum_ways_rec(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(find_target_sum_ways_rec(vec![1], 1), 1);
        assert_eq!(find_target_sum_ways_rec(vec![2, 1], 1), 1);
        assert_eq!(find_target_sum_ways_rec(vec![2, 1], 0), 0);
    }
}
