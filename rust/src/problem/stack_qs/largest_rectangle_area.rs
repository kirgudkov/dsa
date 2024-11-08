// https://leetcode.com/problems/largest-rectangle-in-histogram
// The most naive solution is to run through the array for each element and calculate areas. But it'll take O(n^2) time.
// Monotonic increasing stack is applied here to reduce time complexity down to O(n).
// We'll build up the stack while the entry is grater than the top.
// Once we meet "shorter" bar, we pop k elements that are greater than entry and calculate areas that those popped items could produce 
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut area = 0;
    let mut stack: Vec<(usize, i32)> = vec![];

    // "Insert" 0 as last height by chaining two iterators.
    // It will allow us to finish algorithm without additional loop in case if stack will not be empty at the end.
    heights.iter().chain(&[0]).enumerate().for_each(|(i, &bar)| {
        let mut leftmost_i = i;

        while let Some(&(j, prev_height)) = stack.last() {
            if prev_height > bar {
                area = area.max(prev_height * (i - j) as i32);
                leftmost_i = j;
                stack.pop();
            } else {
                break;
            }
        }

        stack.push((leftmost_i, bar));
    });

    area
}

// Brute force solution:
// 
// for i in 0..heights.len() {
//     let mut min_height = heights[i];
//     for j in i + 1..heights.len() {
//         let width = j - i + 1;
//         min_height = min_height.min(heights[j]);
//         area = area.max(min_height * width as i32);
//     }
// }
//

#[cfg(test)]
mod tests {
    use crate::problem::stack_qs::largest_rectangle_area::largest_rectangle_area;

    #[test]
    fn test_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area(vec![2, 1, 2]), 3);
    }
}