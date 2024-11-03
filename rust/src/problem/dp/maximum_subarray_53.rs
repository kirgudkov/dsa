pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Brute force solution. TC O(N^2)
    // let mut max = i32::MIN;

    // for i in 0..nums.len() {
    //     let mut current = 0;
    //     for j in i..nums.len() {
    //         current += nums[j];
    //         max = max.max(current);
    //     }
    // }

    // max

    let mut cur = nums[0];
    let mut max = nums[0];

    nums.iter().skip(1).for_each(|&num| {
        cur = num.max(cur + num);
        max = max.max(cur);
    });

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }
}