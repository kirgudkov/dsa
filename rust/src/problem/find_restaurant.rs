// https://leetcode.com/problems/minimum-index-sum-of-two-lists
// TC is O(n + m) where n and m are the lengths of list1 and list2 respectively.
// SC is O(n) where n is the length of list1.
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut map = std::collections::HashMap::new();
    let mut res = vec![];
    let mut min = usize::MAX;

    for (i, item) in list1.iter().enumerate() {
        map.insert(item, i);
    }

    for (i, a) in list2.iter().enumerate() {
        if let Some(j) = map.get(&a) {
            match (i + j).cmp(&min) {
                std::cmp::Ordering::Less => {
                    min = i + j;
                    res = vec![a.clone()];
                }
                std::cmp::Ordering::Equal => {
                    res.push(a.clone());
                }
                _ => {}
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_restaurant() {
        assert_eq!(
            find_restaurant(
                vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()],
                vec!["Piatti".to_string(), "The Grill at Torrey Pines".to_string(), "Hungry Hunter Steakhouse".to_string(), "Shogun".to_string()],
            ),
            vec!["Shogun".to_string()]
        );
        assert_eq!(
            find_restaurant(
                vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()],
                vec!["KFC".to_string(), "Shogun".to_string(), "Burger King".to_string()],
            ),
            vec!["Shogun".to_string()]
        );
    }
}