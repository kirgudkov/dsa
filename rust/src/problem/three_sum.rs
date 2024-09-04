pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut result = vec![];

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if let Some(two_sums) = two_sum(&nums[i + 1..], -nums[i]) {
            for two_sum in two_sums {
                result.push(vec![nums[i], two_sum[0], two_sum[1]]);
            }
        }
    }

    result
}

fn two_sum(nums: &[i32], sum: i32) -> Option<Vec<Vec<i32>>> {
    if nums.len() < 2 {
        return None;
    }

    let mut result = vec![];

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if let Ok(k) = nums[i + 1..].binary_search(&(sum - nums[i])) {
            result.push(vec![nums[i], nums[i + 1 + k]]);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::problem::three_sum::three_sum;

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
