use std::fmt::Debug;

use crate::node::{Node, NodePtr};
#[derive(Debug)] 
pub enum Tree<T: Clone + Ord + Debug> {
    Empty,
    Root(NodePtr<T>),
}

impl<T: Clone + Ord + Debug> Tree<T> {
    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn insert(&mut self, data: T) {
        match *self {
            Tree::Empty => {
                *self = Tree::Root(Node::new(data));
            },
            Tree::Root(ref root) => {
                let new_root = Self::insert_rec(root, data);
                *self = Tree::Root(new_root);            
            },
        }
    }

    fn insert_rec(node: &NodePtr<T>, data: T) -> NodePtr<T>  {
        let mut node_borrow = node.borrow_mut();
        if data < node_borrow.data {
            if let Some(ref left) = node_borrow.left {
                let new_left = Self::insert_rec(left, data);
                node_borrow.left = Some(new_left);            
            } else {
                node_borrow.left = Some(Node::new(data));
            }
        } else if data > node_borrow.data {
            if let Some(ref right) = node_borrow.right {
                let new_right = Self::insert_rec(right, data);
                node_borrow.right = Some(new_right);
            } else {
                node_borrow.right = Some(Node::new(data));
            }
        }
        drop(node_borrow);

        Node::update_height(node);

        Self::rebalance(node.clone())
    }

    fn rebalance(node: NodePtr<T>) -> NodePtr<T> {
        let balance = Node::balance_factor(&node);
        // Left heavy subtree
        if balance > 1 {
            let left_balance = Node::balance_factor(&node.borrow().left.as_ref().unwrap());
            if left_balance < 0 {
                let left = node.borrow().left.clone().unwrap();
                node.borrow_mut().left = Some(Self::rotate_left(left));
            }
            return Self::rotate_right(node);
        }
        // Right heavy subtree
        else if balance < -1 {
            let right_balance = Node::balance_factor(&node.borrow().right.as_ref().unwrap());
            if right_balance > 0 {
                let right = node.borrow().right.clone().unwrap();
                node.borrow_mut().right = Some(Self::rotate_right(right));
            }
            return Self::rotate_left(node);
        }
        node
    }

    fn rotate_left(node: NodePtr<T>) -> NodePtr<T> {
        let node_right = node.borrow().right.as_ref().unwrap().clone();
        let node_right_left = node_right.borrow().left.clone();

        node_right.borrow_mut().left = Some(node.clone());
        node.borrow_mut().right = node_right_left;

        Node::update_height(&node);
        Node::update_height(&node_right);

        node_right
    }

    fn rotate_right(node: NodePtr<T>) -> NodePtr<T> {
        let node_left = node.borrow().left.as_ref().unwrap().clone();
        let node_left_right = node_left.borrow().right.clone();

        node_left.borrow_mut().right = Some(node.clone());
        node.borrow_mut().left = node_left_right;

        Node::update_height(&node);
        Node::update_height(&node_left);

        node_left
    }


    pub fn delete(&mut self, data: T) {
        if let Tree::Root(ref root) = *self {
            let new_root = Self::delete_rec(root, data);
            *self = Tree::Root(new_root);
        }
    }

    fn delete_rec(node: &NodePtr<T>, data: T) -> NodePtr<T> {
        let mut node_borrow = node.borrow_mut();

        if data < node_borrow.data {
            if let Some(ref left) = node_borrow.left {
                node_borrow.left = Some(Self::delete_rec(left, data));
            }
        } else if data > node_borrow.data {
            if let Some(ref right) = node_borrow.right {
                node_borrow.right = Some(Self::delete_rec(right, data));
            }
        } else {
            // Node with only one child or no child
            if node_borrow.left.is_none() {
                let temp = node_borrow.right.clone();
                return temp.unwrap_or_else(|| Node::new(data));
            } else if node_borrow.right.is_none() {
                let temp = node_borrow.left.clone();
                return temp.unwrap();
            }

            // Node with two children: Get the inorder successor (smallest in the right subtree)
            let temp = Self::min_value_node(node_borrow.right.as_ref().unwrap().clone());
            node_borrow.data = temp.borrow().data.clone();
            node_borrow.right = Some(Self::delete_rec(&node_borrow.right.clone().unwrap(), temp.borrow().data.clone()));
        }

        drop(node_borrow);
        Node::update_height(node);
        Self::rebalance(node.clone())
    }

    fn min_value_node(node: NodePtr<T>) -> NodePtr<T> {
        let mut current = node;
        loop {
            let next = { current.borrow().left.clone() };
            match next {
                Some(left) => current = left,
                None => break,
            }
        }
        current
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_into_empty_tree() {
        let mut tree = Tree::new();
        tree.insert(10);
        if let Tree::Root(node) = tree {
            assert_eq!(node.borrow().data, 10);
        } else {
            panic!("Tree was expected to have a root");
        }
    }

    #[test]
    fn test_insert_lesser_than_root() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5); 
        if let Tree::Root(node) = &tree {
            if let Some(left_child) = &node.borrow().left {
                assert_eq!(left_child.borrow().data, 5);
            } else {
                panic!("Left child was expected");
            }
        } else {
            panic!("Tree was expected to have a root");
        }
    }

    #[test]
    fn test_insert_greater_than_root() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(15);
        if let Tree::Root(node) = &tree {
            if let Some(right_child) = &node.borrow().right {
                assert_eq!(right_child.borrow().data, 15);
            } else {
                panic!("Right child was expected");
            }
        } else {
            panic!("Tree was expected to have a root");
        }
    }

    #[test]
    fn test_insert_duplicate_value() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(10);
        if let Tree::Root(node) = &tree {
            assert!(node.borrow().left.is_none(), "Left child should not exist for a duplicate value");
            assert!(node.borrow().right.is_none(), "Right child should not exist for a duplicate value");
        } else {
            panic!("Tree was expected to have a root");
        }
    }

    #[test]
    fn test_insert_multiple_values() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(17);

        if let Tree::Root(node) = &tree {
            assert_eq!(node.borrow().data, 10);
            if let Some(left_child) = &node.borrow().left {
                assert_eq!(left_child.borrow().data, 5);
                if let Some(left_left_child) = &left_child.borrow().left {
                    assert_eq!(left_left_child.borrow().data, 3);
                } else {
                    panic!("A left-left child was expected");
                }
                if let Some(left_right_child) = &left_child.borrow().right {
                    assert_eq!(left_right_child.borrow().data, 7);
                } else {
                    panic!("A left-right child was expected");
                }
            } else {
                panic!("A left child was expected");
            }

            if let Some(right_child) = &node.borrow().right {
                assert_eq!(right_child.borrow().data, 15);
                if let Some(right_left_child) = &right_child.borrow().left {
                    assert_eq!(right_left_child.borrow().data, 12);
                } else {
                    panic!("A right-left child was expected");
                }
                if let Some(right_right_child) = &right_child.borrow().right {
                    assert_eq!(right_right_child.borrow().data, 17);
                } else {
                    panic!("A right-right child was expected");
                }
            } else {
                panic!("A right child was expected");
            }
        } else {
            panic!("Tree was expected to have a root");
        }
    }
}
