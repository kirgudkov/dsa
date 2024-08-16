pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // We could optimize lower boundary with: 
    // (piles.iter().map(|&x| x as f64).sum::<f64>() / h as f64).ceil() as i32;
    // but it looks too intricate and doesn't make a big difference tc-wise;
    let mut l = 1;

    // Upper boundary is always the max pile. For example, for piles: [500, 1, 2] and h: 3,
    // Koko has no choice but eat 500 bananas per hour even if other piles are significantly smaller;
    let mut r = *piles.iter().max().unwrap();

    while l < r {
        let m = l + (r - l) / 2;

        // Calculate total hours needed to eat all piles at speed 'm'
        let hours_needed: i32 = piles.iter()
            .map(|&x| (x as f64 / m as f64).ceil() as i32)
            .sum();

        // If we can eat all piles within h hours, try a lower speed
        if hours_needed <= h {
            r = m;
        } else {
            // If we can't eat all piles within 'h' hours, try a higher speed
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
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}