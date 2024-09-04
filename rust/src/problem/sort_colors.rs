// https://leetcode.com/problems/sort-colors/description/
// Counting sort approach
pub fn sort_colors(nums: &mut [i32]) {
    let mut counts: Vec<usize> = vec![0; *nums.iter().max().unwrap() as usize + 1];

    for &num in nums.iter() {
        counts[num as usize] += 1;
    }

    // Calculate the prefix sum of the counts
    // For example, if the counts are [2, 2, 2], then the prefix sum will be [2, 4, 6]
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    let mut sorted = vec![0; nums.len()];

    // Iterate the array in reverse order. Fill the sorted array with the elements:
    // For [2, 0, 2, 1, 1, 0], the counts will be [2, 2, 2], and the prefix sum will be [2, 4, 6]
    // -> [-, -, -, -, -, -] -> [-, 0, -, -, -, -] -> [-, 0, -, 1, -, -] -> [-, 0, 1, 1, -, -] 
    // -> [-, 0, 1, 1, -, 2] -> [0, 0, 1, 1, -, 2] -> [0, 0, 1, 1, 2, 2]
    for &num in nums.iter().rev() {
        sorted[counts[num as usize] - 1] = num;
        counts[num as usize] -= 1;
    }

    nums.copy_from_slice(&sorted);
}

#[cfg(test)]
mod tests {
    use crate::problem::sort_colors::sort_colors;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);

        let mut nums = vec![1, 0, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);

        let mut nums = vec![1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1]);
    }
}
