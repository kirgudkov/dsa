use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/furthest-building-you-can-reach
pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    if heights.len() == 1 {
        return 0;
    }

    let mut heap = BinaryHeap::new();

    for i in 0..heights.len() - 1 {
        let diff = heights[i + 1] - heights[i];

        // go next if flat or lower
        if diff <= 0 {
            continue;
        }

        // If we have ladders, spend them first
        if ladders > 0 {
            heap.push(Reverse(diff));
            ladders -= 1;

            continue;
        }

        // Now we need to decide whether we're going to spend bricks or undo some ladder 
        if heap.peek().is_none() || diff < heap.peek().unwrap().0 {
            bricks -= diff;
        } else {
            bricks -= heap.pop().unwrap().0;
            heap.push(Reverse(diff));
        }

        if bricks < 0 {
            return i as i32;
        }
    }

    (heights.len() - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_building() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        assert_eq!(furthest_building(heights, 5, 1), 4);

        let heights = vec![14, 3, 19, 3];
        assert_eq!(furthest_building(heights, 17, 0), 3);

        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        assert_eq!(furthest_building(heights, 10, 2), 7);

        assert_eq!(furthest_building(vec![1], 0, 0), 0);
    }
}