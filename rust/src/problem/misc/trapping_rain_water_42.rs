// https://leetcode.com/problems/trapping-rain-water
pub fn trap(heights: Vec<i32>) -> i32 {
    let max_height = heights.iter()
        .enumerate()
        .max_by_key(|&(_, h)| h)
        .unwrap();

    let mut moving_max = 0;
    let mut result = 0;

    heights.iter()
        .take(max_height.0)
        .for_each(|&h| {
            result += (moving_max.min(*max_height.1) - h).max(0);
            moving_max = moving_max.max(h);
        });

    moving_max = 0;

    heights.iter()
        .rev()
        .take(heights.len() - max_height.0)
        .for_each(|&h| {
            result += (moving_max.min(*max_height.1) - h).max(0);
            moving_max = moving_max.max(h);
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
