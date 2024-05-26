// https://leetcode.com/problems/query-kth-smallest-trimmed-number/description/
// Description suggested to use radix sort, but I didn't figure out how to apply it.
pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];

    queries.iter().for_each(|query| {
        result.push(trim_and_pick_kth_smallest(&nums, query[0], query[1]));
    });

    result
}

fn trim_and_pick_kth_smallest(nums: &[String], k: i32, trim: i32) -> i32 {
    let mut nums = nums.to_vec();

    nums.iter_mut().for_each(|item| {
        *item = item
            .chars()
            .skip(item.len() - trim as usize)
            .collect();
    });

    let mut items: Vec<(&String, usize)> = nums
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();

    items.sort_by(|a, b| a.0.cmp(b.0));
    items.get((k - 1) as usize).unwrap().1 as i32
}

#[cfg(test)]
mod tests {
    use crate::problem::smallest_trimmed_numbers::smallest_trimmed_numbers;

    #[test]
    fn test_smallest_trimmed_numbers() {
        let nums = ["102", "473", "251", "814"]
            .to_vec()
            .iter()
            .map(|x| x.to_string())
            .collect();

        let queries = vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]];
        assert_eq!(smallest_trimmed_numbers(nums, queries), vec![2, 2, 1, 0]);

        let nums = ["24", "37", "96", "04"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let queries = vec![vec![2, 1], vec![2, 2]];
        assert_eq!(smallest_trimmed_numbers(nums, queries), vec![3, 0]);
    }
}
