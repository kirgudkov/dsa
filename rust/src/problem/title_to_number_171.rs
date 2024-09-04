// https://leetcode.com/problems/excel-sheet-column-number
// Essentially, this problem boils down to converting one notation to another. 
// In this particular example from 26 to decimal. The formula for that conversion generally is:
// decimal = SUM(d[i] * b^j) where:
// d[i] is a digit in i-th position, 
// i changes from n-1 to 0 (moving from LSD to MSD),
// b is a notaion base (2, 10, 16 etc. In our case 26), 
// j changes from 0 to n-1 RTL digit position (units place, tens place etc.);
fn title_to_number(column_title: String) -> i32 {
    column_title.chars().rev().enumerate().fold(0, |sum, (i, ch)| {
        sum + (ch as i32 - 'A' as i32 + 1) * 26i32.pow(i as u32)
    })
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
