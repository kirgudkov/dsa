#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(node1), Some(node2)) => {
            if node1.val < node2.val {
                Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::merge_two_sorted_lists::{ListNode, merge_two_lists};

    #[test]
    fn test_merge_two_lists() {
        let list1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };

        let list2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };

        let merged = merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));
        let expected = vec![1, 1, 2, 3, 4, 4];
        let mut result = Vec::new();
        let mut current = merged;

        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }

        assert_eq!(result, expected);
    }
}
