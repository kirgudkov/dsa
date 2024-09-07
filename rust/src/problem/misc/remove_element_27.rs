// https://leetcode.com/problems/remove-element
pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut n = nums.len() as i32;

    let mut l = 0;
    let mut r = n - 1;

    while l <= r {
        if nums[l as usize] == val {
            nums.swap(l as usize, r as usize);
        }

        if nums[r as usize] == val {
            n -= 1;
            r -= 1;
        } else {
            l += 1
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        assert_eq!(remove_element(&mut [3, 2, 2, 3], 3), 2);
        assert_eq!(remove_element(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
        assert_eq!(remove_element(&mut [1], 1), 0);
        assert_eq!(remove_element(&mut [], 1), 0);
        assert_eq!(remove_element(&mut [0, 4, 4, 0, 4, 4, 4, 0, 2], 4), 4);
    }
}