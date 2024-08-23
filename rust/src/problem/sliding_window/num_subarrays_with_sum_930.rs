use std::collections::HashMap;

// https://leetcode.com/problems/binary-subarrays-with-sum/ 
fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let at_most = |goal: i32, nums: &[i32]| -> i32 {
        let mut l = 0;
        let mut acc = 0;
        let mut count = 0;

        for (r, num) in nums.iter().enumerate() {
            acc += num;

            // ">" is the key;
            // We allow limit exceeding to count trailing elements that do not meet creteria;
            // For example: for [1,0,1,0,1] and goal 2, we won't skip [1,0,1,0] subarray with trailing 0;
            while l <= r && acc > goal {
                acc -= nums[l];
                l += 1;
            }

            count += 1 + r - l;
        }

        count as i32
    };

    at_most(goal, &nums) - at_most(goal - 1, &nums)
}

// cursed
fn num_subarrays_with_sum_ps(nums: Vec<i32>, goal: i32) -> i32 {
    let mut freq = HashMap::new();
    let mut current = 0;
    let mut total = 0;

    for num in &nums {
        current += num;

        if current == goal {
            total += 1;
        }

        if let Some(s) = freq.get(&(current - goal)) {
            total += s;
        }

        freq.entry(current)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(num_subarrays_with_sum(vec![0, 1, 1, 1, 1], 3), 3);
        assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(num_subarrays_with_sum_ps(vec![0, 1, 1, 1, 1], 3), 3);
        assert_eq!(num_subarrays_with_sum_ps(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(num_subarrays_with_sum_ps(vec![0, 0, 0, 0, 0], 0), 15);
    }
}