// https://leetcode.com/problems/daily-temperatures
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack = vec![];

    for (curr_i, &current_temp) in temperatures.iter().enumerate() {
        // Before we push this day onto the stack (since we haven't found the next warmer day for it yet),
        // we should check if this day is the next warmer day for the previous ones:
        while let Some(&prev_i) = stack.last() {
            // If it is, we pop previous day from the stack
            // because we have just found the nearest warmer day that on curr_i - prev_i distance;
            if temperatures[prev_i] < current_temp {
                stack.pop();
                result[prev_i] = (curr_i - prev_i) as i32;
            } else {
                break;
            }
        }

        stack.push(curr_i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
