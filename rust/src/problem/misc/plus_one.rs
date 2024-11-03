pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;

    for i in (0..digits.len()).rev() {
        digits[i] += carry;
        carry = digits[i] / 10;
        digits[i] %= 10;

        if carry == 0 {
            return digits;
        }
    }

    digits.push(1);
    let last = digits.len() - 1;
    digits.swap(0, last);

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
