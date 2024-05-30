// Naive approach; TC is O(n * m)
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![-1; nums1.len()];

    for (i, num) in nums1.iter().enumerate() {
        let mut j = nums2.iter().position(|x| x == num).unwrap() + 1;

        while j < nums2.len() {
            if nums2[j] > *num {
                result[i] = nums2[j];
                break;
            }

            j += 1;
        }
    }

    result
}

// Stack approach; TC is O(n + m)
pub fn next_greater_element2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    let mut stack: Vec<i32> = vec![];

    for num in nums2 {
        while !stack.is_empty() && *stack.last().unwrap() < num {
            map.insert(stack.pop().unwrap(), num);
        }

        stack.push(num);
    }

    nums1.iter().map(|num| *map.get(num).unwrap_or(&-1)).collect()
}

#[cfg(test)]
mod tests {
    use crate::problem::next_greater_element::{next_greater_element, next_greater_element2};

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]), vec![-1, 3, -1]);
        assert_eq!(next_greater_element(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
    }

    #[test]
    fn test_next_greater_element2() {
        assert_eq!(next_greater_element2(vec![4, 1, 2], vec![1, 3, 4, 2]), vec![-1, 3, -1]);
        assert_eq!(next_greater_element2(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
    }
}
