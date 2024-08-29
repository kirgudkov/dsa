use std::cmp::Ordering::{Equal, Less};

// https://leetcode.com/problems/minimum-absolute-difference/description/
fn minimum_abs_difference(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    input.sort_unstable();

    let mut result = vec![];
    let mut min_diff = i32::MAX;

    input.windows(2).for_each(|w| {
        let diff = (w[1] - w[0]).abs();

        match diff.cmp(&min_diff) {
            Less => {
                min_diff = diff;
                result = vec![vec![w[0], w[1]]];
            }
            Equal => {
                result.push(vec![w[0], w[1]]);
            }
            _ => {}
        }
    });

    result
}

// Naive approach: sort, run through once to find min diff and then run once more to find same pairs
pub fn minimum_abs_difference_2(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    input.sort();

    let mut min = i32::MAX;

    let mut i = 0;
    let mut j = 1;

    while j < input.len() {
        if (input[j] - input[i]).abs() < min {
            min = (input[j] - input[i]).abs();
        }

        i += 1;
        j += 1;
    }

    i = 0;
    j = 1;

    while j < input.len() {
        if (input[j] - input[i]).abs() == min {
            result.push(vec![input[i], input[j]]);
        }

        i += 1;
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_abs_difference() {
        let arr = vec![4, 2, 1, 3];
        let result = minimum_abs_difference_2(arr);
        assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);

        let arr = vec![1, 3, 6, 10, 15];
        let result = minimum_abs_difference_2(arr);
        assert_eq!(result, vec![vec![1, 3]]);

        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let result = minimum_abs_difference_2(arr);
        assert_eq!(result, vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);

        let arr = vec![40, 11, 26, 27, -20];
        let result = minimum_abs_difference_2(arr);
        assert_eq!(result, vec![vec![26, 27]]);

        // v2
        let arr = vec![4, 2, 1, 3];
        let result = minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);

        let arr = vec![1, 3, 6, 10, 15];
        let result = minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 3]]);

        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let result = minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);

        let arr = vec![40, 11, 26, 27, -20];
        let result = minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![26, 27]]);
    }
}
