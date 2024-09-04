// https://leetcode.com/problems/query-kth-smallest-trimmed-number/description/
// Description suggested to use radix sort, but I didn't figure out how to apply it.
pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries.iter().map(|query| {
        trim_and_pick_kth_smallest(&nums, query[0], query[1])
    }).collect()
}

fn trim_and_pick_kth_smallest(nums: &[String], k: i32, trim: i32) -> i32 {
    let mut trimmed = nums.to_vec().iter().enumerate().map(|(i, num)| {
        (num.split_at(num.len() - trim as usize).1.to_string(), i)
    }).collect::<Vec<_>>();

    trimmed.sort_unstable();
    trimmed.get((k - 1) as usize).unwrap().1 as i32
}

#[cfg(test)]
mod tests {
    use crate::problem::smallest_trimmed_numbers::smallest_trimmed_numbers;

    #[test]
    fn test_smallest_trimmed_numbers() {
        let nums = vec!["102".to_string(), "473".to_string(), "251".to_string(), "814".to_string()];
        let queries = vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]];
        assert_eq!(smallest_trimmed_numbers(nums, queries), vec![2, 2, 1, 0]);

        let nums = vec!["24".to_string(), "37".to_string(), "96".to_string(), "04".to_string()];
        let queries = vec![vec![2, 1], vec![2, 2]];
        assert_eq!(smallest_trimmed_numbers(nums, queries), vec![3, 0]);
    }
}
