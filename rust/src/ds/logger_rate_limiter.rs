use std::collections::HashMap;

#[derive(Default)]
struct Logger {
    map: HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Self::default()
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(next_allowed) = self.map.get_mut(&message) {
            if timestamp < *next_allowed {
                return false;
            }

            *next_allowed = timestamp + 10;
        } else {
            self.map.insert(message, timestamp + 10);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(1, "foo".to_string()));
        assert!(logger.should_print_message(2, "bar".to_string()));
        assert!(!logger.should_print_message(3, "foo".to_string()));
        assert!(!logger.should_print_message(8, "bar".to_string()));
        assert!(!logger.should_print_message(10, "foo".to_string()));
        assert!(logger.should_print_message(11, "foo".to_string()));
    }
}