// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// O((s + p) log p)
pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut res = vec![0; spells.len()];
    potions.sort_unstable();

    let bs = |spell: i64| {
        let mut l = 0;
        let mut r = potions.len() as i32;

        while l < r {
            let m = l + (r - l) / 2;

            if potions[m as usize] as i64 * spell >= success {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    };

    for (i, &spell) in spells.iter().enumerate() {
        if ((1.0 * success as f64) / spell as f64).ceil() > *potions.last().unwrap() as f64 {
            continue;
        }

        res[i] = potions.len() as i32 - bs(spell as i64);
    }

    res
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