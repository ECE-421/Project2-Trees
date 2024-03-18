use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;


pub type NodePtr<T: Clone> = Rc<RefCell<Node<T>>>;

#[derive(Debug)] 
pub struct Node<T: Clone + Debug> {
    pub data: T,
    pub height: i32,
    pub left: Option<NodePtr<T>>,
    pub right: Option<NodePtr<T>>,
}

impl<T: Ord + Clone + Debug> Node<T> {
    pub fn new(data: T) -> NodePtr<T> {
        Rc::new(RefCell::new(Node {
            data,
            height: 1, 
            left: None,
            right: None,
        }))
    }

    pub fn update_height(node: &NodePtr<T>) {
        let node_borrow = node.borrow();
        let left_height = node_borrow.left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node_borrow.right.as_ref().map_or(0, |n| n.borrow().height);
        drop(node_borrow);
        node.borrow_mut().height = 1 + std::cmp::max(left_height, right_height);
    }

    pub fn balance_factor(node: &NodePtr<T>) -> i32 {
        let node_borrow = node.borrow();
        let left_height = node_borrow.left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node_borrow.right.as_ref().map_or(0, |n| n.borrow().height);
        left_height - right_height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let data = 10;
        let node = Node::new(data);
        let node_borrow = node.borrow();

        assert_eq!(node_borrow.data, data);
        assert_eq!(node_borrow.height, 1);
        assert!(node_borrow.left.is_none());
        assert!(node_borrow.right.is_none());
    }

    #[test]
    fn test_update_height_single_child() {
        let parent = Node::new(10);
        let child = Node::new(5);

        // Simulate adding a left child
        parent.borrow_mut().left = Some(Rc::clone(&child));

        Node::update_height(&parent);

        assert_eq!(parent.borrow().height, 2);
    }

    #[test]
    fn test_update_height_two_children() {
        let parent = Node::new(10);
        let left_child = Node::new(5);
        let right_child = Node::new(15);

        parent.borrow_mut().left = Some(Rc::clone(&left_child));
        parent.borrow_mut().right = Some(Rc::clone(&right_child));

        right_child.borrow_mut().right = Some(Node::new(20));

        Node::update_height(&right_child);
        Node::update_height(&parent);

        assert_eq!(parent.borrow().height, 3);
    }

    #[test]
    fn test_balance_factor() {
        let parent = Node::new(10);
        let left_child = Node::new(5);
        let right_child = Node::new(15);

        parent.borrow_mut().left = Some(Rc::clone(&left_child));
        parent.borrow_mut().right = Some(Rc::clone(&right_child));

        right_child.borrow_mut().right = Some(Node::new(20));
        Node::update_height(&left_child);
        Node::update_height(&right_child);
        Node::update_height(&parent);

        let balance_factor = Node::balance_factor(&parent);
        assert_eq!(balance_factor, -1);
    }
}