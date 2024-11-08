pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let mut longest = 1;
    let mut current = 1;

    nums.windows(2).for_each(|pair| {
        if pair[1] != pair[0] {
            if pair[1] == pair[0] + 1 {
                current += 1;
            } else {
                longest = longest.max(current);
                current = 1;
            }
        }
    });

    longest.max(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }
}