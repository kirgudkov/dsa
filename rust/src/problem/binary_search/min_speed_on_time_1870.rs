pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let mut l = 1;
    let mut r = 10000000;

    if dist.len() as f64 > hour.ceil() {
        return -1;
    }

    while l < r {
        let m = l + (r - l) / 2;

        let time = dist.iter().enumerate().map(|(i, &x)| {
            if i == dist.len() - 1 {
                x as f64 / m as f64
            } else {
                (x as f64 / m as f64).ceil()
            }
        }).sum::<f64>();

        if time <= hour {
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
        assert_eq!(min_speed_on_time(vec![1, 3, 2], 6.0), 1);
        assert_eq!(min_speed_on_time(vec![1, 3, 2], 2.7), 3);
        assert_eq!(min_speed_on_time(vec![1, 3, 2], 1.9), -1);
        assert_eq!(min_speed_on_time(vec![1, 1, 100000], 2.01), 10000000);
    }
}