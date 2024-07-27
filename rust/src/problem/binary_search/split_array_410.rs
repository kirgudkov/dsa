pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let fit = |sum: i32| -> bool {
        let mut subs = 1;
        let mut curr = 0;

        for &num in &nums {
            curr += num;

            if curr > sum {
                subs += 1;
                curr = num;
            }
        }

        subs <= k
    };

    let mut l = *nums.iter().max().unwrap();
    let mut r = nums.iter().sum::<i32>();

    while l < r {
        let m = l + (r - l) / 2;

        if fit(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(split_array(vec![1, 4, 4], 3), 4);
    }
}