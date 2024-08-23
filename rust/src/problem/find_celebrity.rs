// https://leetcode.com/problems/find-the-celebrity
struct Solution {
    persons: Vec<Vec<i32>>,
}

impl Solution {
    // Genius solution from leetcode;
    // The intuition is that if there is a celebrity, we eventually come across the candidate that doesn't know anyone after it.
    // That is, we only need to make sure that this candidate doesn't know anyone before it, so we traverse one more time.
    // Moreover, by definition, everyone knows the celebrity, so we also need to make sure that everyone knows the candidate.
    pub fn find_celebrity(&self, n: i32) -> i32 {
        if n <= 1 {
            return -1;
        }

        let mut celebrity = 0;

        for i in 1..n {
            if self.knows(celebrity, i) {
                celebrity = i;
            }
        }

        for j in 0..n {
            if j == celebrity {
                continue;
            }

            if self.knows(celebrity, j) || !self.knows(j, celebrity) {
                return -1;
            }
        }

        celebrity
    }

    fn knows(&self, a: i32, b: i32) -> bool {
        self.persons[a as usize][b as usize] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_celebrity() {
        let solution = Solution {
            persons: vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
            ],
        };

        assert_eq!(solution.find_celebrity(4), 2);

        let solution = Solution {
            persons: vec![
                vec![1, 1, 0],
                vec![0, 1, 0],
                vec![1, 1, 1],
            ],
        };

        assert_eq!(solution.find_celebrity(3), 1);

        let solution = Solution {
            persons: vec![
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![0, 1, 1],
            ],
        };

        assert_eq!(solution.find_celebrity(3), -1);
    }
}