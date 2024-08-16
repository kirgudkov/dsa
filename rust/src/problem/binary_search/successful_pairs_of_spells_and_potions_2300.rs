// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// O((s + p) log p)
pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut pairs = vec![0; spells.len()];
    potions.sort_unstable();

    let bs = |spell: i64| {
        let mut l = 0;
        let mut r = potions.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if potions[m as usize] as i64 * spell >= success {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        l
    };

    let strongest_potion = *potions.last().unwrap() as f64;

    for (i, &spell) in spells.iter().enumerate() {
        // Pair is successful only when spell * potion >= success,
        // thus we can skip spell, if even strongest potion cannot give a successful pair;
        if (success as f64 / spell as f64).ceil() > strongest_potion {
            continue;
        }

        // potions are sorted; bs finds the index of the first pertinent potion;
        // "count of successful pairs for i-th spell is the length of potions subarray from bs(spell) to the end"
        pairs[i] = potions.len() as i32 - bs(spell as i64);
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
        assert_eq!(result, vec![4, 0, 3]);

        let result = successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16);
        assert_eq!(result, vec![2, 0, 2]);

        let result = successful_pairs(vec![15, 8, 19], vec![38, 36, 23], 328);
        assert_eq!(result, vec![3, 0, 3]);
    }
}