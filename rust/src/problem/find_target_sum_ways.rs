// https://leetcode.com/problems/target-sum/
// TC is O(2^n) and SC is O(n);
// Stack dfs approach is applied as a part of Queue & Stack Leetcode explore card.
//
// There is a better approach using DP with TC O(n * sum) and SC O(n * sum);
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut stack = vec![(0, 0)];
    let mut count = 0;

    while let Some((pos, sum)) = stack.pop() {
        if pos == nums.len() {
            if sum == target {
                count += 1;
            }
        } else {
            stack.push((pos + 1, sum + nums[pos]));
            stack.push((pos + 1, sum - nums[pos]));
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::problem::find_target_sum_ways::find_target_sum_ways;

    #[test]
    fn test() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
        assert_eq!(find_target_sum_ways(vec![2, 1], 1), 1);
        assert_eq!(find_target_sum_ways(vec![2, 1], 0), 0);
    }
}
