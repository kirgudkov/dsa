// https://leetcode.com/problems/minimum-absolute-difference/description/
// Naive approach: sort, run through once to find min diff and then run once more to find same pairs
pub fn minimum_abs_difference(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    // todo: sort manually
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

    // Alternatively we could leverage rust's powerful iterators:
    // result = input.windows(2)
    //     .filter(|x| (x[1] - x[0]).abs() == min)
    //     .map(|x| vec![x[0], x[1]])
    //     .collect();

    result
}

// More performant version
pub fn minimum_abs_difference_v2(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut min = i32::MAX;

    // todo: sort manually
    input.sort_unstable();

    for i in 0..input.len() - 1 {
        let temp = (input[i + 1] - input[i]).abs();

        match temp.cmp(&min) {
            std::cmp::Ordering::Less => {
                min = temp;
                result = vec![vec![input[i], input[i + 1]]];
            }
            std::cmp::Ordering::Equal => {
                result.push(vec![input[i], input[i + 1]]);
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_abs_difference() {
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

        // v2
        let arr = vec![4, 2, 1, 3];
        let result = minimum_abs_difference_v2(arr);
        assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);

        let arr = vec![1, 3, 6, 10, 15];
        let result = minimum_abs_difference_v2(arr);
        assert_eq!(result, vec![vec![1, 3]]);

        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let result = minimum_abs_difference_v2(arr);
        assert_eq!(result, vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);

        let arr = vec![40, 11, 26, 27, -20];
        let result = minimum_abs_difference_v2(arr);
        assert_eq!(result, vec![vec![26, 27]]);
    }
}
