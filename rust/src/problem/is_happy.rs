pub fn is_happy(n: i32) -> bool {
    let mut hs = std::collections::HashSet::new();
    let mut q = std::collections::VecDeque::from([n]);

    fn get_digits(mut k: i32) -> Vec<i32> {
        let mut res = vec![];

        while k > 0 {
            res.push(k % 10);
            k /= 10;
        }

        res
    }

    while !q.is_empty() {
        let num = q.pop_front().unwrap();

        if hs.contains(&num) {
            return false;
        }

        hs.insert(num);

        let digits = get_digits(num);
        let new_num = digits.iter().fold(0, |acc, x| acc + x * x);

        if new_num == 1 {
            return true;
        }

        q.push_back(new_num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(is_happy(19));
        assert!(!is_happy(2));
    }
}