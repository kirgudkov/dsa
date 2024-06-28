pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut points = std::collections::HashMap::new();
    let mut max = 0;

    for num in nums {
        *points.entry(num).or_insert(0) += 1;
        max = max.max(num);
    }

    let mut max_points = vec![0; max as usize + 1];
    max_points[1] = *points.get(&1).unwrap_or(&0);

    for num in 2..max + 1 {
        max_points[num as usize] = std::cmp::max(
            max_points[num as usize - 1],
            max_points[num as usize - 2] + (points.get(&num).unwrap_or(&0) * num),
        )
    }

    max_points[max as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_earn() {
        assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}