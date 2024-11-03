// TC - O(n); SC - O(n), where n is the length of the input string
pub fn calculate(s: String) -> i32 {
    let mut operands = vec![];
    let mut operations = vec![];

    let chars = s.chars()
        .collect::<Vec<_>>();

    let mut i = chars.len() as i32 - 1;

    while i >= 0 {
        match chars[i as usize] {
            ' ' => i -= 1,
            '+' | '-' | '*' | '/' => {
                operations.push(chars[i as usize]);
                i -= 1;
            }
            ')' => {
                let mut j = i;
                while j >= 0 && chars[j as usize] != '(' {
                    j -= 1;
                }

                operands.push(calculate(s[j as usize + 1..i as usize].to_string()));
                i = j - 1;
            }
            '0'..='9' => {
                let mut j = i;
                while j >= 0 && chars[j as usize].is_ascii_digit() {
                    j -= 1;
                }

                operands.push(s[(j + 1) as usize..=i as usize].parse::<i32>().unwrap());
                i = j;
            }
            char => panic!("Unsupported character: {char}")
        }
    }

    let mut stash: Option<(i32, char)> = None; // (lhs, operation)

    while let Some(operation) = operations.pop() {
        match operation {
            '+' | '-' => {
                let lhs = operands.pop().unwrap();

                // Addition and Subtraction can be performed only if the next operation doesn't have higher priority;
                // In that case we stash left operand and operation:
                if matches!(operations.last().unwrap_or(&'?'), '/' | '*') {
                    stash = Some((lhs, operation));
                    continue;
                }

                let rhs = operands.pop().unwrap();
                operands.push(execute(operation, lhs, rhs));
            }
            '*' | '/' => {
                // Multiplication and Division can be performed immediately
                let lhs = operands.pop().unwrap();
                let rhs = operands.pop().unwrap();
                let result = execute(operation, lhs, rhs);

                operands.push(result);

                // If we have something stashed, put it back to the stach,
                // most likely previous operation result now is the rhs operand for the stashed expression
                if let Some((lhs, operation)) = stash {
                    operands.push(lhs);
                    operations.push(operation);
                }
            }
            _ => panic!("Unsupported operation: {operation}")
        }
    }

    operands[0]
}

fn execute(operation: char, lhs: i32, rhs: i32) -> i32 {
    match operation {
        '/' => lhs / rhs,
        '*' => lhs * rhs,
        '+' => lhs + rhs,
        '-' => lhs - rhs,
        _ => panic!("Unsupported operation: {operation}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calculate("42".to_string()), 42);
        assert_eq!(calculate("3+2*2".to_string()), 7);
        assert_eq!(calculate(" 3/2 ".to_string()), 1);
        assert_eq!(calculate(" 3+5 / 2 ".to_string()), 5);
        assert_eq!(calculate("(1+2)*5/3+6/4*2".to_string()), 7);
    }
}