pub fn int_to_roman(num: i32) -> String {
    let mut result = String::new();

    let digit_to_roman = |digit: usize, map: &(char, char, char)| match digit {
        9 => format!("{}{}", map.0, map.2),
        4 => format!("{}{}", map.0, map.1),
        _ => map.1.to_string().repeat(digit / 5) + &map.0.to_string().repeat(digit % 5)
    };

    let ones_map = ('I', 'V', 'X');
    let tens_map = ('X', 'L', 'C');
    let hund_map = ('C', 'D', 'M');

    (0..num / 1000).for_each(|_| { // thousands
        result.push(hund_map.2);
    });

    result.push_str(&digit_to_roman(num as usize % 1000 / 100, &hund_map));
    result.push_str(&digit_to_roman(num as usize % 100 / 10, &tens_map));
    result.push_str(&digit_to_roman(num as usize % 10, &ones_map));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    }
}
