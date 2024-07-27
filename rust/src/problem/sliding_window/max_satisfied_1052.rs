// https://leetcode.com/problems/grumpy-bookstore-owner
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let mut max = (i32::MIN, 0, 0);
    let mut curr = 0;

    // Searchin for the best window that contains max customers on grumpy minutes
    while r < customers.len() {
        if grumpy[r] == 1 {
            curr += customers[r];
        }

        r += 1;

        if r - l < minutes as usize {
            continue;
        }

        if curr > max.0 {
            max = (curr, l, r - 1);
        }

        if grumpy[l] == 1 {
            curr -= customers[l];
        }

        l += 1;
    }

    // Count all non-grumpy
    let mut res = 0;

    for (i, c) in customers.iter().enumerate() {
        if (i < max.1 || i > max.2) && grumpy[i] == 0 {
            res += *c;
        }
    }

    // Add our best window
    for &cus in customers.iter().skip(max.1).take(max.2 - max.1 + 1) {
        res += cus;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3), 16);
        assert_eq!(max_satisfied(vec![4, 10, 10], vec![1, 1, 0], 2), 24);
    }
}
