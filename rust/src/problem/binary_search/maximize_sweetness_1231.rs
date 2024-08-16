// https://leetcode.com/problems/divide-chocolate/
pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
    let splits = |max| {
        let mut pieces = 0;
        let mut curr = 0;

        for &s in &sweetness {
            curr += s;

            if curr >= max {
                pieces += 1;
                curr = 0;
            }
        }

        pieces > k
    };

    let mut l = 0;
    let mut r = sweetness.iter().sum::<i32>();

    while l < r {
        let m = l + (r - l) / 2;

        if splits(m) {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(maximize_sweetness(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5), 6);
        assert_eq!(maximize_sweetness(vec![5, 6, 7, 8, 9, 1, 2, 3, 4], 8), 1);
        assert_eq!(maximize_sweetness(vec![1, 2, 2, 1, 2, 2, 1, 2, 2], 2), 5);
    }
}
