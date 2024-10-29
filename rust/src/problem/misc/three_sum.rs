// If sorting isn't allowed
pub fn three_sum_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = std::collections::HashSet::<Vec<i32>>::new();
    let mut duplicates = std::collections::HashSet::<i32>::new();

    for i in 0..nums.len() {
        let mut seen = std::collections::HashSet::<i32>::new();

        if !duplicates.contains(&nums[i]) {
            duplicates.insert(nums[i]);

            for j in i + 1..nums.len() {
                let complement = -nums[i] - nums[j];
                if seen.contains(&complement) {
                    let mut vec = vec![nums[i], nums[j], complement];
                    vec.sort_unstable();
                    result.insert(vec);
                } else {
                    seen.insert(nums[j]);
                }
            }
        }
    }

    result.into_iter().collect()
}

// If sorting is allowed:
pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    nums.sort_unstable();

    for i in 0..nums.len() {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }

        for j in i + 1..nums.len() {
            if j > i + 1 && nums[j - 1] == nums[j] {
                continue;
            }
            if let Ok(k) = nums[j + 1..].binary_search(&(-nums[i] - nums[j])) {
                result.push(vec![nums[i], nums[j], nums[j + 1 + k]]);
            }
        }
    }

    result
}

pub fn three_sum_3(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    nums.sort_unstable();

    for (i, &a) in nums.iter().enumerate() {
        if i > 0 && a == nums[i - 1] {
            continue;
        }

        if let Some(pairs) = two_sum(&nums[i + 1..], -a) {
            for [b, c] in pairs {
                result.push(vec![a, b, c]);
            }
        }
    }

    result
}

fn two_sum(nums: &[i32], sum: i32) -> Option<Vec<[i32; 2]>> {
    if nums.len() < 2 {
        return None;
    }

    let mut result = vec![];

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if let Ok(k) = nums[i + 1..].binary_search(&(sum - nums[i])) {
            result.push([nums[i], nums[i + 1 + k]]);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::three_sum::three_sum_3;

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum_3(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(three_sum_3(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(three_sum_3(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
