pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut len = nums.len();

    let mut r: i32 = nums.len() as i32 - 1;
    let mut l: i32 = 0;

    while l <= r {
        if nums[r as usize] == val {
            len -= 1;
            r -= 1;
            continue;
        }

        if nums[l as usize] == val {
            nums.swap(l as usize, r as usize);
            len -= 1;
            r -= 1;
        } else {
            l += 1;
        }
    }

    len as i32
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