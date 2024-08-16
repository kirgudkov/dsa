// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days
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

    // Search range is [min_capacity, max_capacity];
    // The best option is to have days count == packages count; 
    // In that case we could ship 1 package each day, thus min capacity is the max package weight;
    let mut l = *weights.iter().max().unwrap();
    // The worst option is to have only one day to ship everything, thus the max capacity is the sum of all packages;
    let mut r = weights.iter().sum::<i32>();

    // Now we have to find optimal capacity to deliver everything in n days
    while l <= r {
        let m = l + (r - l) / 2;

        // If m capacity allows us to deliver everything in time, 
        // then we can try a smaller capacity
        if feasible(m) {
            r = m - 1;
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