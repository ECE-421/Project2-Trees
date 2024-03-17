use std::fmt::Debug;

use crate::node::{Node, NodePtr};
#[derive(Debug)]
pub enum Tree<T: Clone + Ord> {
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
            }
            Tree::Root(ref root) => {
                let new_root = Self::insert_rec(root, data);
                *self = Tree::Root(new_root);
            }
        }
    }

    fn insert_rec(node: &NodePtr<T>, data: T) -> NodePtr<T> {
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
        *self = match *self {
            Tree::Empty => Tree::Empty,
            Tree::Root(ref root) => {
                match Self::delete_rec(&root, &data) {
                    Some(node) => Tree::Root(node),
                    None => Tree::Empty,
                }
            }
        };
    }

    fn delete_rec(node: &NodePtr<T>, data: &T) -> Option<NodePtr<T>> {
        let mut node_borrow = node.borrow_mut();
        if data < &node_borrow.data {
            if let Some(ref left) = node_borrow.left {
                node_borrow.left = Self::delete_rec(left, data);
            }
        } else if data > &node_borrow.data {
            if let Some(ref right) = node_borrow.right {
                node_borrow.right = Self::delete_rec(right, data);
            }
        } else {
            if node_borrow.left.is_none() && node_borrow.right.is_none() {
                return None;
            } else if node_borrow.left.is_none() {
                return node_borrow.right.clone();
            } else if node_borrow.right.is_none() {
                return node_borrow.left.clone();
            } else {
                let successor = Self::min_value_node(node_borrow.right.as_ref().unwrap().clone());
                node_borrow.data = successor.borrow().data.clone();
                node_borrow.right = Self::delete_rec(&node_borrow.right.clone().unwrap(), &successor.borrow().data);
            }
        }
        drop(node_borrow);
        Node::update_height(node);
        
        Some(Self::rebalance(node.clone()))
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

    ///Return the number of leaves in the tree
    fn leaves(&self) -> usize {
        match *self {
            Tree::Empty => 0,
            Tree::Root(ref root) => {
                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    1
                } else {
                    //If node has children, return sum of recursive call of 'leaves' on children

                    let mut sum = 0;

                    //Check if both left and right children are Some
                    if root.borrow().left.is_some() && root.borrow().right.is_some() {
                        sum += Tree::Root(root.borrow().left.clone().unwrap()).leaves() + Tree::Root(root.borrow().right.clone().unwrap()).leaves();
                    }

                    //Check if only left child is Some
                    if root.borrow().left.is_some() && root.borrow().right.is_none() {
                        sum += Tree::Root(root.borrow().left.clone().unwrap()).leaves();
                    }

                    //Check if only right child is Some
                    if root.borrow().left.is_none() && root.borrow().right.is_some() {
                        sum += Tree::Root(root.borrow().right.clone().unwrap()).leaves();
                    }

                    return sum;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool{
        match *self {
            Tree::Empty => true,
            Tree::Root(_) => false,
        }
    }

    pub fn print_in_order_traversal(&self) { 
        match *self {
            Tree::Empty => (),
            Tree::Root(ref root) => {
                if let Some(left) = &root.borrow().left {
                    Tree::Root(left.clone()).print_in_order_traversal();
                }
                println!("{:?}", root.borrow().data);
                if let Some(right) = &root.borrow().right {
                    Tree::Root(right.clone()).print_in_order_traversal();
                }
            }
        }
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
            assert!(
                node.borrow().left.is_none(),
                "Left child should not exist for a duplicate value"
            );
            assert!(
                node.borrow().right.is_none(),
                "Right child should not exist for a duplicate value"
            );
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

    #[test]
    fn test_leaves_count(){
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
        // println!("{:#?}", tree);
        assert_eq!(tree.leaves(), 3);
    }

    #[test]
    fn test_is_tree_empty(){
        let mut tree = Tree::new();
        assert_eq!(tree.is_empty(), true);
        tree.insert(2);
        assert_eq!(tree.is_empty(), false);
        tree.delete(2);
        // println!("{:#?}", tree);
        assert_eq!(tree.is_empty(), true);
    }
    
    #[test]
    fn test_in_order_print(){
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(17);
        tree.insert(4);
        tree.insert(16);
        tree.print_in_order_traversal();
    }
}
