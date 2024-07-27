pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let feasible = |capacity: i32| -> bool {
        let mut d = 1;
        let mut load = 0;

        for &weight in &weights {
            load += weight;

            if load > capacity {
                d += 1;
                load = weight;
            }
        }

        d <= days
    };

    let mut l = *weights.iter().max().unwrap();
    let mut r = weights.iter().sum::<i32>();

    while l < r {
        let m = l + (r - l) / 2;

        if feasible(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        assert_eq!(ship_within_days(weights, days), 3);

        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        assert_eq!(ship_within_days(weights, days), 6);

        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        assert_eq!(ship_within_days(weights, days), 15);
    }
}