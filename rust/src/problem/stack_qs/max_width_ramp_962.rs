pub fn max_width_ramp_n2(nums: Vec<i32>) -> i32 {
    let mut max_width = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] <= nums[j] {
                max_width = max_width.max(j - i);
            }
        }
    }

    max_width as i32
}

pub fn max_width_ramp_n(nums: Vec<i32>) -> i32 {
    let mut max_width = 0;
    let mut stack = vec![];

    for (i, &num) in nums.iter().enumerate() {
        if stack.last().map(|&j| nums[j] > num).unwrap_or(true) {
            stack.push(i);
        }
    }
    
    for (i, &num) in nums.iter().enumerate().rev() {
        while stack.last().map(|&j| nums[j] <= num).unwrap_or(false) {
            max_width = max_width.max(i - stack.pop().unwrap());
        }
    }

    max_width as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_width_ramp_n2(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(max_width_ramp_n2(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
        assert_eq!(max_width_ramp_n2(vec![6, 0, 8, 2, 1, 5]), 4);

        assert_eq!(max_width_ramp_n(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(max_width_ramp_n(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
        assert_eq!(max_width_ramp_n(vec![6, 0, 8, 2, 1, 5]), 4);
    }
}