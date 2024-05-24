use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let ones_vocab: HashMap<i32, &str> = [
        (1, "I"),
        (5, "V"),
        (10, "X"),
    ].iter().cloned().collect();

    let tens_vocab: HashMap<i32, &str> = [
        (1, "X"),
        (5, "L"),
        (10, "C"),
    ].iter().cloned().collect();

    let hundreds_vocab: HashMap<i32, &str> = [
        (1, "C"),
        (5, "D"),
        (10, "M"),
    ].iter().cloned().collect();

    let thousands_vocab: HashMap<i32, &str> = [
        (1, "M"),
    ].iter().cloned().collect();

    let mut roman = String::new();

    let ones = num % 10;
    let tens = num % 100 / 10;
    let hundreds = num % 1000 / 100;
    let thousands = num / 1000;

    for _ in 0..thousands {
        roman.push_str(thousands_vocab[&1]);
    }

    roman.push_str(&digit_to_roman(hundreds, &hundreds_vocab));
    roman.push_str(&digit_to_roman(tens, &tens_vocab));
    roman.push_str(&digit_to_roman(ones, &ones_vocab));

    roman
}

fn digit_to_roman(digit: i32, vocab: &HashMap<i32, &str>) -> String {
    let mut roman = String::new();

    match digit {
        9 => roman.push_str((String::new() + vocab[&1] + vocab[&10]).as_str()),
        4 => roman.push_str((String::new() + vocab[&1] + vocab[&5]).as_str()),
        _ => {
            if digit >= 5 {
                roman.push_str(vocab[&5]);
            }
            for _ in 0..digit % 5 {
                roman.push_str(vocab[&1]);
            }
        }
    }

    roman
}

// println!("\nInteger to roman");
// println!("3749: {}", crate::integer_to_roman::int_to_roman(3749));
// println!("58: {}", crate::integer_to_roman::int_to_roman(58));
// println!("1994: {}", crate::integer_to_roman::int_to_roman(1994));
