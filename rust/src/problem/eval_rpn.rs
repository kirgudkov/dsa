pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
    if tokens.len() == 1 {
        return tokens[0].parse().unwrap();
    }

    solve(&mut tokens)
}

fn solve(tokens: &mut Vec<String>) -> i32 {
    if let Some(operation) = Operation::from_str(tokens.pop().as_deref()) {
        return perform(operation, get_operand(tokens), get_operand(tokens));
    }

    0
}

fn perform(operation: Operation, right: i32, left: i32) -> i32 {
    match operation {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide => left / right,
    }
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(s: Option<&str>) -> Option<Operation> {
        match s {
            Some("+") => Some(Operation::Add),
            Some("-") => Some(Operation::Subtract),
            Some("*") => Some(Operation::Multiply),
            Some("/") => Some(Operation::Divide),
            _ => None,
        }
    }

    fn is_operation(s: &str) -> bool {
        matches!(s, "+" | "-" | "*" | "/")
    }
}

fn get_operand(tokens: &mut Vec<String>) -> i32 {
    if Operation::is_operation(tokens.last().unwrap()) {
        solve(tokens)
    } else {
        tokens.pop().unwrap().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::eval_rpn::eval_rpn;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]), 9);
        assert_eq!(eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]), 6);
        assert_eq!(eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()]), 22);
        assert_eq!(eval_rpn(vec!["18".to_string()]), 18);
    }
}
