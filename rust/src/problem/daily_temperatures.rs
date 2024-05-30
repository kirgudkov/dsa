pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = vec![];

    for (i, t) in temperatures.iter().enumerate() {
        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < *t {
            let j = stack.pop().unwrap();
            result[j] = (i - j) as i32;
        }
        stack.push(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::daily_temperatures::daily_temperatures;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
