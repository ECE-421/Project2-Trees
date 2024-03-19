use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::iter::Successors;
use std::rc::Rc;
use std::{clone, fmt};

use crate::main;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

impl NodeColor {
    // Method to recolor the enum to the opposite color
    fn flip_color(color: NodeColor) -> NodeColor {
        match color {
            NodeColor::Red => NodeColor::Black,
            NodeColor::Black => NodeColor::Red,
        }
    }
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;

#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>,
    pub left: RedBlackTree<T>,
    pub right: RedBlackTree<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedBlackTreeSet<T: Ord + Display + Debug + Copy> {
    pub root: RedBlackTree<T>,
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T>
where
    T: Ord + Display + Debug + Clone + Copy,
{
    pub fn new() -> Self {
        RedBlackTreeSet { root: None }
    }

    pub fn find(&mut self, key: T) -> RedBlackTree<T> {
        let root = self.root.clone();
        self.find_recursion(&root, key)
    }

    pub fn find_recursion(&mut self, node: &RedBlackTree<T>, key: T) -> RedBlackTree<T> {
        match node {
            Some(node) => {
                if key < node.borrow().key {
                    self.find_recursion(&node.borrow().left, key)
                } else if key > node.borrow().key {
                    self.find_recursion(&node.borrow().right, key)
                } else {
                    Some(node.clone())
                }
            }
            None => None,
        }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red, // New nodes are always red initially
            key,
            parent: None,
            left: None,
            right: None,
        }));

        if let Some(root) = &self.root {
            self.insert_recursive(root.clone(), new_node.clone());
            self.fix(new_node.clone());
        } else {
            // If the tree is empty, make the new node the root and color it black
            new_node.borrow_mut().color = NodeColor::Black;
            self.root = Some(new_node);
        }
    }

    fn insert_recursive(&mut self, node: Tree<T>, new_node: Tree<T>) {
        let mut node_borrowed = node.borrow_mut();

        if new_node.borrow().key < node_borrowed.key {
            if let Some(left) = &node_borrowed.left {
                self.insert_recursive(left.clone(), new_node);
            } else {
                node_borrowed.left = Some(Rc::clone(&new_node));
                new_node.borrow_mut().parent = Some(node.clone());
            }
        } else {
            if let Some(right) = &node_borrowed.right {
                self.insert_recursive(right.clone(), new_node);
            } else {
                node_borrowed.right = Some(Rc::clone(&new_node));
                new_node.borrow_mut().parent = Some(node.clone());
            }
        }
    }

    fn fix(&mut self, new_node: Tree<T>) {
        // Refernces https://www.codesdope.com/course/data-structures-red-black-trees-insertion/
        // used the pseudo code to implement the rust algorith for fixing an insert
        let is_root = new_node.borrow().parent.is_none();
        println!("inserted Node: {:?}", new_node.borrow().key);

        let mut parent_color = new_node
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .clone()
            .color;
        let mut node_ref = new_node.borrow().clone();

        while node_ref.clone().parent.is_some()
            && node_ref.parent.as_ref().unwrap().borrow().clone().color == NodeColor::Red
        {
            let node = new_node.clone();

            // Find the parent node
            // it is safe to use unwrap here since we already verified the parent has a value
            let mut parent = node_ref.parent.clone().unwrap();

            //parent is a left child
            if self.is_left_child(&parent).is_some() {
                //find grand_parent
                let grandparent = match parent.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.clone()),
                    None => None,
                };

                //find uncle
                // if uncle exists it must be right child of gp
                let uncle_ref = match grandparent {
                    Some(ref grandparent_node) => grandparent_node.borrow().right.clone(),
                    None => None,
                };

                if uncle_ref.is_some()
                    && uncle_ref.as_ref().unwrap().borrow().color == NodeColor::Red
                {
                    //flip uncle colour
                    let uncle = uncle_ref.as_ref().unwrap().clone();

                    let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                    {
                        uncle.borrow_mut().color = new_uncle_colour;
                    }

                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour =
                            NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                        }

                        node_ref = grandparent.borrow().clone();
                    }
                } else {
                    // new node is a right child
                    if self.is_left_child(&new_node).is_none() {
                        let parent_clone = node_ref.parent.clone().unwrap();
                        // node_ref = parent_clone.borrow().clone();
                        // let test = node_ref.parent.as_ref().unwrap().borrow().clone();
                        self.rotate_left(parent.clone());
                        // let test1 = node_ref.parent.as_ref().unwrap().borrow().clone();
                        node_ref = parent_clone.borrow().clone();
                        parent = parent_clone.borrow().parent.as_ref().unwrap().clone();
                    }

                    // set parent color to black
                    {
                        parent.borrow_mut().color = NodeColor::Black;
                    }

                    // set grandparent color to black
                    // since we may have reassigned parent we should reget the grandparent

                    let grandparent_clone = match parent.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.clone()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent_clone {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_right(grandparent.clone());
                    }
                }
            } else {
                //find grand_parent
                let grandparent = match parent.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.clone()),
                    None => None,
                };

                //find uncle
                // if uncle exists it must be right child of gp
                let uncle_ref = match grandparent {
                    Some(ref grandparent_node) => grandparent_node.borrow().left.clone(),
                    None => None,
                };

                if uncle_ref.is_some()
                    && uncle_ref.as_ref().unwrap().borrow().color == NodeColor::Red
                {
                    //flip uncle colour
                    let uncle = uncle_ref.as_ref().unwrap().clone();

                    let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                    {
                        uncle.borrow_mut().color = new_uncle_colour;
                    }

                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour =
                            NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                        }

                        node_ref = grandparent.borrow().clone();
                    }
                } else {
                    // new node is a right child
                    if Some(true) == self.is_left_child(&new_node) {
                        let parent_clone = node_ref.parent.clone().unwrap();
                        // node_ref = parent_clone.borrow().clone();
                        // let test = node_ref.parent.as_ref().unwrap().borrow().clone();
                        self.rotate_right(parent.clone());
                        node_ref = parent_clone.borrow().clone();
                        parent = parent_clone.borrow().parent.as_ref().unwrap().clone();
                    }

                    // set parent color to black
                    parent.borrow_mut().color = NodeColor::Black;

                    // since we may have reassigned parent we should reget the grandparent

                    let grandparent_clone = match parent.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.clone()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent_clone {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_left(grandparent.clone());
                    }
                }
            }
            // parent_color = node_ref.color;
        }

        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
    }

    fn is_left_child(&self, node: &Tree<T>) -> Option<bool> {
        if let Some(ref parent) = node.borrow().parent.clone() {
            if let Some(ref left_child) = parent.borrow().left.clone() {
                if (left_child.borrow().key == node.borrow().key) {
                    Some(true)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn rotate_left(&mut self, x: Tree<T>) {
        let mut y = x.borrow_mut().right.take().unwrap();
        x.borrow_mut().right = y.borrow_mut().left.take();

        if let Some(ref mut left) = y.borrow_mut().left {
            left.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x.borrow().parent.clone();

        if let Some(parent) = x.borrow().parent.clone() {
            if x.borrow().key < parent.borrow().key {
                parent.borrow_mut().left = Some(y.clone());
            } else {
                parent.borrow_mut().right = Some(y.clone());
            }
        } else {
            self.root = Some(y.clone());
        }

        y.borrow_mut().left = Some(x.clone());
        x.borrow_mut().parent = Some(y.clone());
    }

    pub fn rotate_right(&mut self, y: Tree<T>) {
        let mut x = y.borrow_mut().left.take().unwrap();
        y.borrow_mut().left = x.borrow_mut().right.take();

        if let Some(ref mut right) = x.borrow_mut().right {
            right.borrow_mut().parent = Some(y.clone());
        }

        x.borrow_mut().parent = y.borrow().parent.clone();

        if let Some(parent) = y.borrow().parent.clone() {
            if y.borrow().key < parent.borrow().key {
                parent.borrow_mut().left = Some(x.clone());
            } else {
                parent.borrow_mut().right = Some(x.clone());
            }
        } else {
            self.root = Some(x.clone());
        }

        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().parent = Some(x.clone());
    }

    pub fn delete(&mut self, key: T) {
        //follow pseudo code at https://www.codesdope.com/course/data-structures-red-black-trees-deletion/
        let mut x: RedBlackTree<T>;
        let found_node = self.find(key);
        let found_node_ref = found_node;

        let found = found_node_ref.as_ref().unwrap().borrow();
        let z_left = found_node_ref.as_ref().unwrap().borrow().left.clone();
        let z_right = found_node_ref.as_ref().unwrap().borrow().right.clone();

        let y = match found_node_ref.clone() {
            Some(node_ref) => Some(node_ref.clone()),
            None => {
                print!("Key not found");
                return;
            }
        };

        let mut y_orginal_color = y.as_ref().unwrap().borrow().color.clone();

        if z_left.is_none() {
            x = z_right;
            self.transplant(&found_node_ref.clone(), &x.clone())
        } else if z_right.is_none() {
            x = z_left.clone();
            self.transplant(&found_node_ref.clone(), &x.clone())
        } else {
            // node has 2 children
            let y = self.find_minimum(&z_right.clone());
            y_orginal_color = y.as_ref().unwrap().borrow().color.clone();

            //x = y.right
            x = y.as_ref().unwrap().borrow().clone().right;

            if y.as_ref()
                .unwrap()
                .borrow()
                .clone()
                .parent
                .as_ref()
                .unwrap()
                .borrow()
                .clone()
                .key
                == found_node_ref.as_ref().unwrap().borrow().clone().key
            {
                if x.is_some() {
                    x.as_ref().unwrap().borrow_mut().parent = y.clone();
                }
            } else {
                self.transplant(&y.clone(), &y.as_ref().unwrap().borrow().right.clone());
                y.as_ref().unwrap().borrow_mut().right = z_right;
                y.as_ref()
                    .unwrap()
                    .borrow()
                    .right
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .parent = y.clone();
            }
            self.transplant(&found_node_ref.clone(), &y.clone());
            y.as_ref().unwrap().borrow_mut().left = z_left.clone();
            y.as_ref().unwrap().borrow_mut().parent = y.clone();
            y.as_ref().unwrap().borrow_mut().color =
                found_node_ref.as_ref().unwrap().borrow().color.clone();
        }
        // if y_orginal_color == NodeColor::Black {
        //     self.fix_delete(x.as_ref().unwrap().clone());
        // }
    }

    fn fix_delete(&mut self, x: Tree<T>) {
        let mut x_ref = x.borrow().clone();
        // w = sibling of x
        // while  x_ref.parent.is_some() && x_ref.color == NodeColor::Black {
        // x is the left child of parent
        let x_parent = x_ref.parent.clone();

        if Some(true) == self.is_left_child(&x) {
            // w is the sibling of x
            let sibling_ref = match x_parent {
                Some(ref sibling_node) => sibling_node.borrow().right.clone(),
                None => None,
            };

            if sibling_ref.is_some() {
                //case 1 sibling is red
                let mut sibling = sibling_ref.as_ref().unwrap().clone();

                if sibling.borrow().color == NodeColor::Red {
                    sibling.borrow_mut().color = NodeColor::Black;
                    x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    self.rotate_left(x_parent.as_ref().unwrap().clone());
                    sibling = x_parent
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .clone();
                } else if sibling.borrow().left.is_some() && sibling.borrow().right.is_some() {
                    // sibling has 2 children
                    if sibling.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black
                        && sibling.borrow().right.as_ref().unwrap().borrow().color
                            == NodeColor::Black
                    {
                        sibling.borrow_mut().color = NodeColor::Red;
                        x_ref = x_parent.as_ref().unwrap().borrow().clone();
                    }
                } else if sibling.borrow().right.as_ref().unwrap().borrow().color
                    == NodeColor::Black
                {
                    sibling.borrow().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    sibling.borrow_mut().color = NodeColor::Red;
                    self.rotate_right(sibling);
                    sibling = x_parent
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .clone();
                }

                sibling.borrow_mut().color = x_parent.as_ref().unwrap().borrow().color.clone();
                x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                sibling.borrow().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                self.rotate_left(x_parent.as_ref().unwrap().clone());
                x_ref = self.root.as_ref().unwrap().borrow().clone();
            }
        } else {
            // x is the right child
            // w is the sibling of x
            let sibling_ref = match x_parent {
                Some(ref sibling_node) => sibling_node.borrow().left.clone(),
                None => None,
            };

            if sibling_ref.is_some() {
                //case 1 sibling is red
                let mut sibling = sibling_ref.as_ref().unwrap().clone();

                if sibling.borrow().color == NodeColor::Red {
                    sibling.borrow_mut().color = NodeColor::Black;
                    x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    self.rotate_right(x_parent.as_ref().unwrap().clone());
                    sibling = x_parent
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .clone();
                } else if sibling.borrow().right.is_some() && sibling.borrow().left.is_some() {
                    // sibling has 2 children
                    if sibling.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black
                        && sibling.borrow().left.as_ref().unwrap().borrow().color
                            == NodeColor::Black
                    {
                        sibling.borrow_mut().color = NodeColor::Red;
                        x_ref = x_parent.as_ref().unwrap().borrow().clone();
                    }
                } else if sibling.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black
                {
                    sibling.borrow().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    sibling.borrow_mut().color = NodeColor::Red;
                    self.rotate_left(sibling);
                    sibling = x_parent
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .clone();
                }

                sibling.borrow_mut().color = x_parent.as_ref().unwrap().borrow().color.clone();
                x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                sibling.borrow().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                self.rotate_right(x_parent.as_ref().unwrap().clone());
                x_ref = self.root.as_ref().unwrap().borrow().clone();
            }
        }
    }

    fn transplant(&mut self, u: &RedBlackTree<T>, v: &RedBlackTree<T>) {
        // pesudo code from https://www.codesdope.com/course/data-structures-red-black-trees-deletion/

        let u_node = u.as_ref().unwrap().clone();
        let u_parent = u.as_ref().unwrap().borrow().parent.clone();
        //check if u is root
        if u_parent.is_none() {
            self.root = v.clone();
        } else if Some(true) == self.is_left_child(&u_node.clone()) {
            u_parent.as_ref().unwrap().borrow_mut().left = v.clone();
        } else {
            u_parent.as_ref().unwrap().borrow_mut().right = v.clone();
        }

        // Set the parent of v to be the parent of u
        if let Some(v_node) = v.clone() {
            v_node.borrow_mut().parent = u_parent.clone();
        }
    }

    pub fn find_minimum(&mut self, tree: &RedBlackTree<T>) -> RedBlackTree<T> {
        let root = self.root.clone();
        self.find_minimum_recursion(&tree.clone())
    }

    pub fn find_minimum_recursion(&mut self, tree: &RedBlackTree<T>) -> RedBlackTree<T> {
        match tree {
            Some(node) => {
                if node.borrow().left.is_none() {
                    println!("minimum on right is: {:?}", node.borrow().key);
                    Some(node.clone())
                } else {
                    self.find_minimum_recursion(&node.borrow().left.clone())
                }
            }
            None => tree.clone(),
        }
    }

    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            println!("Red-Black Tree:");
            self.print_recursive(root.clone(), "", true);
        } else {
            println!("Empty tree");
        }
    }

    fn print_recursive(&self, node: Tree<T>, prefix: &str, is_left: bool) {
        let node_borrowed = node.borrow();

        // Print the node's key and color
        println!(
            "{}{}{:?} ({:?})",
            prefix,
            if is_left {
                "L├── "
            } else {
                "R└── "
            },
            node_borrowed.key,
            node_borrowed.color
        );

        // Calculate the prefix for child nodes
        let mut child_prefix = prefix.to_string();
        child_prefix.push_str(if is_left { "│   " } else { "    " });

        // Recursively print the left subtree
        if let Some(left) = &node_borrowed.left {
            self.print_recursive(left.clone(), &child_prefix, true);
        }

        // Recursively print the right subtree
        if let Some(right) = &node_borrowed.right {
            self.print_recursive(right.clone(), &child_prefix, false);
        }
    }

    pub fn leaves(&self) -> i32 {
        let root = self.root.clone();
        self.leaves_recursion(&root)
    }

    pub fn leaves_recursion(&self, node: &RedBlackTree<T>) -> i32 {
        match node {
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    1
                } else {
                    self.leaves_recursion(&node.borrow().left)
                        + self.leaves_recursion(&node.borrow().right)
                }
            }
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn get_tree_height(&self) -> i32 {
        let root = self.root.clone();
        self.get_tree_height_recursion(&root)
    }

    pub fn get_tree_height_recursion(&self, node: &RedBlackTree<T>) -> i32 {
        match node {
            Some(node) => {
                let left_height = self.get_tree_height_recursion(&node.borrow().left);
                let right_height = self.get_tree_height_recursion(&node.borrow().right);

                if left_height > right_height {
                    left_height + 1
                } else {
                    right_height + 1
                }
            }
            None => 0,
        }
    }

    pub fn print_in_order_traversal(&self) {
        let root = self.root.clone();
        self.print_in_order_traversal_recursion(&root);
    }

    pub fn print_in_order_traversal_recursion(&self, node: &RedBlackTree<T>) {
        match node {
            Some(node) => {
                self.print_in_order_traversal_recursion(&node.borrow().left);
                println!(
                    "Key: {:?}, Color: {:?}",
                    node.borrow().key,
                    node.borrow().color
                );
                self.print_in_order_traversal_recursion(&node.borrow().right);
            }
            None => {}
        }
    }
}
