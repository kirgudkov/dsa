// https://leetcode.com/problems/largest-rectangle-in-histogram
// The most naive solution is to run through the array for each element and calculate areas. But it'll take O(n^2) time.
// Monotonic increasing stack is applied here to reduce time complexity down to O(n).
// We'll build up the stack while the entry is grater than the top.
// Once we meet "shorter" bar, we pop k elements that are greater than entry and calculate areas that those popped items could produce 
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<(usize, i32)> = vec![];

    // "Insert" 0 as last height by chaining two iterators.
    // It will allow us to finish algorithm without additional loop in case if stack will not be empty at the end.
    heights.iter().chain(&[0]).enumerate().for_each(|(next_i, &next_height)| {
        let mut start_index = next_i;

        while let Some(&(prev_index, prev_height)) = stack.last() {
            if next_height >= prev_height {
                break;
            }

            max_area = max_area.max(prev_height * (next_i - prev_index) as i32);
            start_index = prev_index;
            stack.pop();
        }

        stack.push((start_index, next_height));
    });

    max_area
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::largest_rectangle_area::largest_rectangle_area;

    #[test]
    fn test_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }
}