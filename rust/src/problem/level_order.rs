use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::problem::is_same_tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut result = vec![];
    let mut queue = VecDeque::from([root]);

    while !queue.is_empty() {
        let mut len = queue.len();
        let mut level = vec![];

        while len > 0 {
            if let Some(next) = queue.pop_front().unwrap() {
                level.push(next.borrow().val);

                if next.borrow().left.is_some() {
                    queue.push_back(next.borrow().left.clone())
                }

                if next.borrow().right.is_some() {
                    queue.push_back(next.borrow().right.clone())
                }
            }

            len -= 1;
        }

        result.push(level);
    }

    result
}
