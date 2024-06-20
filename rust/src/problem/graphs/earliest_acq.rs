use crate::ds::disjoint_set::DisjointSet;

pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
    logs.sort_unstable_by_key(|k| k[0]);

    let mut ds = DisjointSet::new(n as usize);
    let mut count = n;

    for log in logs {
        if ds.union(log[1] as usize, log[2] as usize) {
            count -= 1;
        }

        if count == 1 {
            return log[0];
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earliest_acq() {
        let logs = vec![
            vec![20190101, 0, 1],
            vec![20190104, 3, 4],
            vec![20190107, 2, 3],
            vec![20190211, 1, 5],
            vec![20190224, 2, 4],
            vec![20190301, 0, 3],
            vec![20190312, 1, 2],
            vec![20190322, 4, 5],
        ];
        assert_eq!(earliest_acq(logs, 6), 20190301);

        let logs = vec![
            vec![0, 2, 0],
            vec![1, 0, 1],
            vec![3, 0, 3],
            vec![4, 1, 2],
            vec![7, 3, 1],
        ];
        assert_eq!(earliest_acq(logs, 4), 3);
    }
}