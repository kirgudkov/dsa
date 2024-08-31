pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut i = digits.len() as i32 - 1;
    let mut carry = 1;

    while i >= 0 && carry > 0 {
        let sum = digits[i as usize] + carry;
        digits[i as usize] = sum % 10;
        carry = sum / 10;
        i -= 1;
    }

    if carry > 0 {
        digits.insert(0, carry);
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
