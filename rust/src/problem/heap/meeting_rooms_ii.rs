// https://leetcode.com/problems/meeting-rooms-ii
// TC is sorting O(n log n) + O(n) for loop * heap poop/push O(log n) => O(n log n)
pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|interval| interval[0]);
    let mut pqueue = std::collections::BinaryHeap::from([
        std::cmp::Reverse(intervals[0][1])
    ]);

    intervals.iter().skip(1).for_each(|interval| {
        if pqueue.peek().unwrap().0 <= interval[0] {
            pqueue.pop();
        }

        pqueue.push(std::cmp::Reverse(interval[1]));
    });

    pqueue.len() as i32
}

pub fn min_meeting_rooms_alt(intervals: Vec<Vec<i32>>) -> i32 {
    let mut starts = intervals.iter()
        .map(|i| i[0])
        .collect::<Vec<i32>>();

    let mut ends = intervals.iter()
        .map(|i| i[1])
        .collect::<Vec<i32>>();

    starts.sort_unstable();
    ends.sort_unstable();

    let (mut i, mut j, mut rooms, mut max_rooms) = (0, 0, 0, 0);

    while i < starts.len() {
        if starts[i] < ends[j] {
            rooms += 1;
            max_rooms = max_rooms.max(rooms);
            i += 1;
        } else {
            rooms -= 1;
            j += 1;
        }
    }

    max_rooms
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


        let intervals = vec![vec![9, 10], vec![4, 9], vec![4, 17]];
        assert_eq!(min_meeting_rooms_alt(intervals), 2);

        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms_alt(intervals), 2);

        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(min_meeting_rooms_alt(intervals), 1);

        let intervals = vec![vec![0, 30], vec![17, 33], vec![5, 10], vec![15, 20]];
        assert_eq!(min_meeting_rooms_alt(intervals), 3);

        let intervals = vec![vec![13, 15], vec![1, 13]];
        assert_eq!(min_meeting_rooms_alt(intervals), 1);
    }
}