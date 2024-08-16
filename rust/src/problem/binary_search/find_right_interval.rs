// https://leetcode.com/problems/find-right-interval
pub fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![-1; intervals.len()];
    let mut hm = std::collections::HashMap::new();

    intervals.iter().enumerate().for_each(|(i, val)| {
        hm.insert(val.clone(), i as i32);
    });

    intervals.sort_unstable_by(|a, b| {
        a[0].cmp(&b[0])
    });

    for i in 0..intervals.len() {
        // N^2 solution: 
        // scan the rest of the intervals, the first one that satisfies the condition is the closes right interval
        // for j in i..intervals.len() {
        //     if intervals[j][0] >= intervals[i][1] {
        //         res[*hm.get(&intervals[i]).unwrap() as usize] = *hm.get(&intervals[j]).unwrap();
        //         break;
        //     }
        // }

        // N logN solution:
        // binary search the rest of the intervals
        res[*hm.get(&intervals[i]).unwrap() as usize] = intervals[i..]
            .binary_search_by(|x| x[0].cmp(&intervals[i][1]))
            .map_or_else(|x| {
                // Err(closest): We didn't find the exact match, but the closes possible position.
                if x + i < intervals.len() {
                    *hm.get(&intervals[x + i]).unwrap()
                } else {
                    -1
                }
            }, |x| {
                // Ok(target): We found the exact match, so it is closest right interval: [1, 3] -> [3, 4]
                *hm.get(&intervals[x + i]).unwrap()
            });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_right_interval() {
        assert_eq!(find_right_interval(vec![vec![4, 5], vec![2, 3], vec![1, 2]]), vec![-1, 0, 1]);
        assert_eq!(find_right_interval(vec![vec![1, 2], vec![2, 3], vec![0, 1], vec![3, 4]]), vec![1, 3, 0, -1]);
        assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
        assert_eq!(find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]), vec![-1, 0, 1]);
        assert_eq!(find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]), vec![-1, 2, -1]);
    }
}
