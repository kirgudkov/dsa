use std::collections::VecDeque;

// https://leetcode.com/problems/largest-rectangle-in-histogram
// Most naive solution is to run through the array for each element and calculate areas. But it'll take O(n^2) time.
// Monotonic increasing stack is applied here to reduce time complexity down to O(n).
// We'll build up stack while the entry is grater than the top.
// Once we meet "shorter" bar, we pop k elements that are greater than entry and calculate areas that those popped items could produce 
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: VecDeque<(usize, i32)> = VecDeque::from([(0, heights[0])]);

    // "Insert" 0 as last height by chaining two iterators.
    // It will allow us to finish algorithm without additional loop in case if stack will not be empty at the end.
    for (i, height) in heights.iter().chain([0].iter()).enumerate() {
        let mut last_popped_i = i;

        while !stack.is_empty() && stack.back().unwrap().1 > *height {
            let popped = stack.pop_back().unwrap();
            max_area = max_area.max(popped.1 * (i - popped.0) as i32);
            last_popped_i = popped.0;
        }

        stack.push_back((last_popped_i, *height));
    }

    max_area
}

#[cfg(test)]
mod tests {
    use crate::problem::largest_rectangle_area::largest_rectangle_area;

    #[test]
    fn test_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }
}