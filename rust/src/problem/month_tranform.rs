fn month_transform(days: usize, offset: usize) -> Vec<usize> {
    let cols = (days as f32 / 7.0).ceil() as usize + if offset > 4 { 1 } else { 0 };
    let mut month = vec![0; cols * 7];

    for row in 0..7 {
        for col in 0..cols {
            if (col == 0 && row < offset) || ((row + 1) + (col * 7) - offset > days) {
                continue;
            }

            month[row * cols + col] = (row + 1) + (col * 7) - offset;
        }
    }

    month
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_transform() {
        assert_eq!(month_transform(31, 1), vec![0, 7, 14, 21, 28, 1, 8, 15, 22, 29, 2, 9, 16, 23, 30, 3, 10, 17, 24, 31, 4, 11, 18, 25, 0, 5, 12, 19, 26, 0, 6, 13, 20, 27, 0]);
        assert_eq!(month_transform(31, 2), vec![0, 6, 13, 20, 27, 0, 7, 14, 21, 28, 1, 8, 15, 22, 29, 2, 9, 16, 23, 30, 3, 10, 17, 24, 31, 4, 11, 18, 25, 0, 5, 12, 19, 26, 0]);
    }
}
