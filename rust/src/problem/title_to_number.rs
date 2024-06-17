pub fn title_to_number(column_title: String) -> i32 {
    let mut result = 0;
    let mut i = 0;

    while i < column_title.len() {
        let ch = column_title.chars().nth(column_title.len() - (i + 1)).unwrap();
        result += 26i32.pow(i as u32) * (ch as i32 - 'A' as i32 + 1);

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
        assert_eq!(title_to_number("FXSHRXW".to_string()), 2147483647);
    }
}