pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();

    while l < r {
        let m = l + (r - l) / 2;

        if piles.iter().map(|&x| (x as f64 / m as f64).ceil() as i32).sum::<i32>() <= h {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(min_eating_speed(piles, h), 4);

        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(min_eating_speed(piles, h), 30);

        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(min_eating_speed(piles, h), 23);
    }
}