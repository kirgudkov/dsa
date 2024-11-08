pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() {
        return true;
    }

    intervals.sort_unstable();

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert!(!can_attend_meetings(intervals));

        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert!(can_attend_meetings(intervals));
    }
}
