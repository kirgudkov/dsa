use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/furthest-building-you-can-reach
pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    if heights.len() == 1 {
        return 0;
    }

    // Min heap to store the smallest height climbed so far
    let mut heap = BinaryHeap::new();

    for i in 0..heights.len() - 1 {
        let height = heights[i + 1] - heights[i];

        // go next if flat or lower
        if height <= 0 {
            continue;
        }

        // If we have ladders, spend them first
        if ladders > 0 {
            heap.push(Reverse(height));
            ladders -= 1;

            continue;
        }

        // Now we need to decide whether we're going to spend bricks or undo last used ladder: 
        // if the current height is less than the min height we've climbed so far,
        // then it's better to keep using bricks
        if heap.peek().is_none() || height < heap.peek().unwrap().0 {
            bricks -= height;
        } else {
            // Otherwise, we could spend less bricks on that min height we've met previouse then the current one
            bricks -= heap.pop().unwrap().0;
            // and spend used ladder on this height
            heap.push(Reverse(height));
        }

        // If it turned out that we ran out of bricks - we've reached as far as we could
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