pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    nums.sort();

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

    let mut result: Vec<Vec<i32>> = vec![];

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

// println!("\nThree sum");
// println!("[-1,0,1,2,-1,-4] -> {:?}", crate::three_sum::three_sum(vec![-1, 0, 1, 2, -1, -4]));
// println!("[0,1,1] -> {:?}", crate::three_sum::three_sum(vec![0, 1, 1]));
// println!("[0,0,0] -> {:?}", crate::three_sum::three_sum(vec![0, 0, 0]));
// println!("[0,0,0,0] -> {:?}", crate::three_sum::three_sum(vec![0, 0, 0, 0]));
