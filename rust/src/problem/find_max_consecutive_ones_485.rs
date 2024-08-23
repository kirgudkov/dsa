// https://leetcode.com/problems/max-consecutive-ones
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut r = 0;

    while r < nums.len() {
        if nums[r] == 1 {
            let l = r;
            while r < nums.len() && nums[r] == 1 {
                r += 1;
            }

            max = max.max(r - l);
        }

        r += 1;
    }

    max as i32
}

// Alternative without nested loop
fn find_max_consecutive_ones_1(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut r = 0;
    let mut l = None;

    while r < nums.len() {
        if nums[r] == 1 {
            if l.is_none() {
                l = Some(r);
            }
        } else {
            l = None;
        }

        if let Some(l) = l {
            max = max.max(r - l + 1);
        }

        r += 1;
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(find_max_consecutive_ones(vec![0, 0, 0, 0, 0, 0]), 0);

        assert_eq!(find_max_consecutive_ones_1(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(find_max_consecutive_ones_1(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}