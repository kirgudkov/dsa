use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/meeting-rooms-ii
// TC is sorting O(n log n) + O(n) for loop * heap poop/push O(log n) => O(n log n)
pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|interval| interval[0]);

    let mut min_heap = BinaryHeap::new();
    // Push the ending time of the first meeting
    min_heap.push(Reverse(intervals[0][1]));

    for interval in intervals.iter().skip(1) {
        // If current interval starts before the meeting that ends first, we need a new room
        if interval[0] < min_heap.peek().unwrap().0 {
            min_heap.push(Reverse(interval[1]));
        } else {
            // Otherwise, previous meeting has ended, so we can "use that room"
            min_heap.pop();
            min_heap.push(Reverse(interval[1]));
        }
    }

    min_heap.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![9, 10], vec![4, 9], vec![4, 17]];
        assert_eq!(min_meeting_rooms(intervals), 2);

        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms(intervals), 2);

        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(min_meeting_rooms(intervals), 1);

        let intervals = vec![vec![0, 30], vec![17, 33], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms(intervals), 3);

        let intervals = vec![vec![13, 15], vec![1, 13]];
        assert_eq!(min_meeting_rooms(intervals), 1);
    }
}