use std::collections::HashSet;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let set = HashSet::<char>::from_iter(jewels.chars());

    stones.chars().fold(0, |mut count, stone| {
        if set.contains(&stone) {
            count += 1;
        }

        count
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        assert_eq!(num_jewels_in_stones(jewels, stones), 3);

        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        assert_eq!(num_jewels_in_stones(jewels, stones), 0);
    }
}