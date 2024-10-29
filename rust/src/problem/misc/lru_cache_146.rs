use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

// This implementation is only for educational purpose!
// Do not try to re-create it in interview. Pick another language :D

#[derive(Default)]
struct LRUListNode {
    key: i32,
    value: i32,
    prev: Option<Weak<RefCell<LRUListNode>>>,
    next: Option<Rc<RefCell<LRUListNode>>>,
}

impl LRUListNode {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        let node = LRUListNode {
            key,
            value,
            ..Default::default()
        };

        Rc::new(RefCell::new(node))
    }
}

type NodeRef = Rc<RefCell<LRUListNode>>;

#[derive(Default)]
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodeRef>,
    least: Option<NodeRef>,
    most: Option<NodeRef>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self { capacity, ..Default::default() }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            let value = node.borrow().value;
            self.update_node(node, None);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            self.update_node(node, Some(value));
        } else {
            self.create_node(key, value);
        }

        if self.map.len() > self.capacity {
            if let Some(least_node) = self.least.clone() {
                self.delete_node(least_node);
            }
        }
    }

    fn update_node(&mut self, node_rc: NodeRef, value: Option<i32>) {
        self.delete_node(node_rc.clone());
        let key = node_rc.borrow().key;
        let val = value.unwrap_or(node_rc.borrow().value);
        self.create_node(key, val);
    }

    fn delete_node(&mut self, node_rc: NodeRef) {
        let key = node_rc.borrow().key;
        self.map.remove(&key);

        let prev = node_rc.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        let next = node_rc.borrow().next.clone();

        if let Some(prev_node) = prev.clone() {
            prev_node.borrow_mut().next = next.clone();
        }

        if let Some(next_node) = next.clone() {
            next_node.borrow_mut().prev = prev.as_ref().map(Rc::downgrade);
        }

        if let Some(least_node) = &self.least {
            if Rc::ptr_eq(least_node, &node_rc) {
                self.least = next.clone();
            }
        }

        if let Some(most_node) = &self.most {
            if Rc::ptr_eq(most_node, &node_rc) {
                self.most = prev.clone();
            }
        }

        // Clear the node's next and prev
        node_rc.borrow_mut().next = None;
        node_rc.borrow_mut().prev = None;
    }

    fn create_node(&mut self, key: i32, value: i32) {
        let node_rc = LRUListNode::new(key, value);

        if let Some(most_node) = self.most.clone() {
            most_node.borrow_mut().next = Some(node_rc.clone());
            node_rc.borrow_mut().prev = Some(Rc::downgrade(&most_node));
        }

        if self.least.is_none() {
            self.least = Some(node_rc.clone());
        }

        self.most = Some(node_rc.clone());
        self.map.insert(key, node_rc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        obj.put(4, 4);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);
    }
}
