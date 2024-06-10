pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; digits.len()];
    let mut carry = 1;

    for (i, &digit) in digits.iter().enumerate().rev() {
        let sum = digit + carry;
        res[i] = sum % 10;
        carry = sum / 10;
    }

    if carry > 0 {
        res.insert(0, carry);
    }

    res
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
