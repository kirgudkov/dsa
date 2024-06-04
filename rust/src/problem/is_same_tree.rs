use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_same_tree_it(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::from([(p, q)]);

    while let Some(pair) = queue.pop_front() {
        if pair.0.is_none() && pair.1.is_none() {
            return false;
        }

        if (pair.0.is_none() && pair.1.is_some()) || (pair.0.is_some() && pair.1.is_none()) {
            return false;
        }

        let p = pair.0.unwrap();
        let q = pair.1.unwrap();

        if p.eq(&q) {
            return true;
        }

        let p = p.borrow();
        let q = q.borrow();

        if p.val != q.val {
            return false;
        }

        queue.push_back((p.left.clone(), q.left.clone()));
        queue.push_back((p.right.clone(), q.right.clone()));
    }

    false
}

pub fn is_same_tree_rec(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }

    if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) {
        return false;
    }

    let q = q.unwrap();
    let p = p.unwrap();

    if p.eq(&q) {
        return true;
    }

    let p = p.borrow();
    let q = q.borrow();

    if p.val != q.val {
        return false;
    }

    is_same_tree_rec(p.left.clone(), q.left.clone()) && is_same_tree_rec(p.right.clone(), q.right.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestFn = fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool;

    fn test_is_same_tree(test_fn: TestFn) {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert!(test_fn(p, q));

        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        assert!(!test_fn(p, q));
    }

    #[test]
    fn test_is_same_tree_it() {
        test_is_same_tree(is_same_tree_it);
    }

    #[test]
    fn test_is_same_tree_rec() {
        test_is_same_tree(is_same_tree_rec);
    }
}