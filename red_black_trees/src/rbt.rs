use std::cell::RefCell;
use std::iter::Successors;
use std::rc::{Rc, Weak};
use std::{clone, fmt};
use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::PartialEq;

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
type Parent<T> = Option<Weak<RefCell<TreeNode<T>>>>;
type RedBlackTree<T> = Option<Tree<T>>;

#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: Parent<T>,
    pub left: RedBlackTree<T>,
    pub right: RedBlackTree<T>,
}


// struct WeakTreeNode<T>(Weak<RefCell<TreeNode<T>>>);

// impl<T: PartialEq> PartialEq for WeakTreeNode<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.upgrade() == other.0.upgrade()
//     }
// }

#[derive(Debug)]
pub struct RedBlackTreeSet<T: Ord+Display+Debug+Copy> where T: Ord+Display+Debug+Clone+Copy+PartialEq{
    pub root: RedBlackTree<T>,
}
impl<T: Ord + fmt::Debug> TreeNode<T> where T: Ord+Display+Debug+Clone+Copy {
    fn new(val: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            key: val,
            parent: None,
            left: None,
            right: None,

        }
    }
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self {
        RedBlackTreeSet { root: None }
    }

    pub fn find(&mut self, key: T) -> RedBlackTree<T>{
        let root = self.root.clone();

        if root.as_ref().unwrap().borrow().key == key{
            return self.root.clone();
        }
        self.find_recursion(&root, key)
    
        
    }

    pub fn find_recursion(&mut self, node: &RedBlackTree<T>, key: T) -> RedBlackTree<T>{
        match node {
            Some(node) => {
                if key < node.borrow().key {
                    self.find_recursion(&node.borrow().left, key)
                } else if key > node.borrow().key {
                    self.find_recursion(&node.borrow().right, key)
                } else {
                    Some(node.clone())
                }
            },
            None => None,
        }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(key)));

        if let Some(mut root) = self.root.clone() {
            self.insert_recursive(&root, &new_node);
            self.fix(new_node.clone());
        } else {
            // If the tree is empty, make the new node the root and color it black
            new_node.borrow_mut().color = NodeColor::Black;
            self.root = Some(new_node);
           
        }
    }

    fn insert_recursive(&mut self, node: &Tree<T>, new_node: &Tree<T>) {
        let mut node_borrowed = node.borrow_mut();

        if new_node.borrow().key < node_borrowed.key {
            if let Some(left) = &node_borrowed.left {
                self.insert_recursive(&left, &new_node);
            } else {
                let weak_node = Rc::downgrade(&node.clone()); // Convert Rc to Weak
                node_borrowed.left = Some(Rc::clone(&new_node));
                new_node.borrow_mut().parent = Some(weak_node); 
            }
        } else {
            if let Some(right) = &node_borrowed.right {
                self.insert_recursive(&right, &new_node);
            } else {
                let weak_node = Rc::downgrade(&node.clone()); // Convert Rc to Weak
                node_borrowed.right = Some(Rc::clone(&new_node));
                new_node.borrow_mut().parent = Some(weak_node); 
            }
        }

    }
    

    fn fix(&mut self, new_node: Tree<T>) {
        // Refernces https://www.codesdope.com/course/data-structures-red-black-trees-insertion/
        // used the pseudo code to implement the rust algorith for fixing an insert

        let is_root = new_node.borrow().parent.is_none();
        println!("Inserted Node: {:?}", new_node.borrow().key);


        let mut node_ref = new_node.clone();
        let test = node_ref.borrow().parent.is_some();
        let t = node_ref.borrow().parent.clone().unwrap().upgrade().unwrap().borrow().key;
 

        while node_ref.borrow().parent.is_some()
            && node_ref.borrow().parent.clone().unwrap().upgrade().unwrap().borrow().color == NodeColor::Red
        {


            // let node = new_node.clone();
            
            // Find the parent node
            // it is safe to use unwrap here since we already verified the parent has a value
            //parent is a left child
            println!("parent key after setting color to red: {:?}", node_ref.borrow().parent.clone().unwrap().upgrade().unwrap().borrow().key);
            let parent = node_ref.borrow().parent.clone().unwrap().upgrade().unwrap().clone();
            println!("parent key after setting color to red: {:?}", parent.borrow().key);
            if Some(true) == self.is_left_child(&parent) {

                 //find grand_parent
                let grandparent = match parent.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                    None => None,
                };

                //find uncle
                // if uncle exists it must be right child of gp
                let uncle_ref = match grandparent {
                    Some(ref grandparent_node) => {
                        grandparent_node.borrow().right.clone()
                    },
                    None => None,
                };

                if uncle_ref.is_some() && uncle_ref.as_ref().unwrap().borrow().color == NodeColor::Red{
                    //flip uncle colour
                    let uncle = uncle_ref.as_ref().unwrap().clone();

                    let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                    {
                        uncle.borrow_mut().color = new_uncle_colour;
                        println!("uncle key after setting color to red: {:?}", uncle.borrow().key);
                    }
                
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                            println!("Grandparent key after setting color to red: {:?}", grandparent.borrow().key);
                            node_ref = grandparent.clone();
                        }
                        
                        // node_ref = grandparent.borrow().clone();
                    }
                    
                } else { 
                    // new node is a right child
                    if Some(false) == self.is_left_child(&new_node) {
                        let parent_weak = new_node.borrow().parent.clone().unwrap();
                        let parent_upgrade = parent_weak.upgrade().unwrap();
                        node_ref = parent_upgrade.clone();
                        println!("Parent key after setting color to black: {:?}", parent_upgrade.borrow().key);
                        self.rotate_left(parent_upgrade);
                    }

                    let parent_weak = node_ref.borrow().parent.clone().unwrap();
                    let parent_upgrade = parent_weak.upgrade().unwrap();

                    parent_upgrade.borrow_mut().color = NodeColor::Black;
                    println!("Parent key after setting color to black: {:?}", parent_upgrade.borrow().key);

                    // Case 3
                    let grandparent = match parent_upgrade.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        println!("Grandparent key after setting color to red: {:?}", grandparent.borrow().key);
                        self.rotate_right(grandparent.clone());
                        // node_ref = grandparent.clone();
                    }

                    
                    
                }
            } else {
                //find grand_parent
                let grandparent = match parent.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                    None => None,
                };

                //find uncle
                // if uncle exists it must be left child of gp
                let uncle_ref = match grandparent {
                    Some(ref grandparent_node) => {
                        grandparent_node.borrow().left.clone()
                    },
                    None => None,
                };

                if uncle_ref.is_some() && uncle_ref.as_ref().unwrap().borrow().color == NodeColor::Red{
                    //flip uncle colour
                    let uncle = uncle_ref.as_ref().unwrap().clone();

                    let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                    {
                        uncle.borrow_mut().color = new_uncle_colour;
                        println!("uncle key after setting color to red: {:?}", uncle.borrow().key);
                    }
                
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                            println!("Grandparent key after setting color to red: {:?}", grandparent.borrow().key);
                            node_ref = grandparent.clone();
                        }
                        
                        // node_ref = grandparent.borrow().clone();
                    }
                    
                } else { 
                    // new node is a right child
                    if Some(true) == self.is_left_child(&new_node) {
                        let parent_weak = new_node.borrow().parent.clone().unwrap();
                        let parent_upgrade = parent_weak.upgrade().unwrap();
                        node_ref = parent_upgrade.clone();
                        println!("Parent key after setting color to black: {:?}", parent_upgrade.borrow().key);
                        self.rotate_right(parent_upgrade);
                    }

                    // Case 3
                    let parent_weak = node_ref.borrow().parent.clone().unwrap();
                    let parent_upgrade = parent_weak.upgrade().unwrap();
                    
                    parent_upgrade.borrow_mut().color = NodeColor::Black;
                    println!("Parent key after setting color to black: {:?}", parent_upgrade.borrow().key);

                    let grandparent = match parent_upgrade.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        println!("Grandparent key after setting color to red: {:?}", grandparent.borrow().key);
                        self.rotate_left(grandparent.clone());
                        // node_ref = grandparent.clone();
                    }
                    
                }
            }
                
           
        }

        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;

    }
    
    
    fn is_left_child(&self, node: &Tree<T>) -> Option<bool> {
        if let Some(parent_weak) = &node.borrow().parent {
            if let Some(parent) = parent_weak.upgrade() {
                if let Some(left_child) = &parent.borrow().left {
                    return Some(Rc::ptr_eq(left_child, node));
                }
            }
        }
        return Some(false)
    }

    pub fn rotate_right(&mut self, x: Tree<T>) {
        let mut y = x.borrow_mut().left.take().unwrap();
        x.borrow_mut().left = y.borrow_mut().right.take();
        
        if let Some(ref mut right) = y.borrow_mut().right {
            // Set parent using weak pointer
            right.borrow_mut().parent = Some(Rc::downgrade(&x));
        }
        
        // Set y's parent using weak pointer
        y.borrow_mut().parent = x.borrow().parent.clone();
        
        if let Some(parent_weak) = x.borrow().parent.clone() {
            // Upgrade parent weak pointer to strong reference
            let parent_upgrade = parent_weak.upgrade().unwrap();
            if x.borrow().key < parent_upgrade.borrow().key {
                parent_upgrade.borrow_mut().left = Some(y.clone());
            } else {
                parent_upgrade.borrow_mut().right = Some(y.clone());
            }
        } else {
            // Update root if x was root
            self.root = Some(y.clone());
        }
        
        // Set y's right child
        y.borrow_mut().right = Some(x.clone());
        // Set x's parent using weak pointer
        x.borrow_mut().parent = Some(Rc::downgrade(&y));
    }
    
    
    

    pub fn rotate_left(&mut self, x: Tree<T>) {
        let mut y = x.borrow_mut().right.take().unwrap();
        x.borrow_mut().right = y.borrow_mut().left.take();
        
        if let Some(ref mut left) = y.borrow_mut().left {
            // Set parent using weak pointer
            left.borrow_mut().parent = x.borrow().parent.clone();
        }
        
        // Set y's parent using weak pointer
        y.borrow_mut().parent = x.borrow().parent.clone();
        
        if let Some(parent_weak) = x.borrow().parent.clone() {
            // Upgrade parent weak pointer to strong reference
            let parent_upgrade = parent_weak.upgrade().unwrap();
            if x.borrow().key < parent_upgrade.borrow().key {
                parent_upgrade.borrow_mut().left = Some(y.clone());
            } else {
                parent_upgrade.borrow_mut().right = Some(y.clone());
            }
        } else {
            // Update root if x was root
            self.root = Some(y.clone());
        }
        
        // Set y's left child
        y.borrow_mut().left = Some(x.clone());
        // Set x's parent using weak pointer
        x.borrow_mut().parent = Some(Rc::downgrade(&y));
    }
    
    

    pub fn delete(&mut self, key: T) {
        let mut x: RedBlackTree<T> = None;
        let found_node = self.find(key);
        let found_node_ref = found_node.clone();
        
        println!("{}", Rc::strong_count(&found_node_ref.as_ref().unwrap()));
        // let found = found_node_ref.as_ref().unwrap().borrow();
        let z_parent = found_node_ref.as_ref().unwrap().borrow().parent.clone();
        let z_left = found_node_ref.as_ref().unwrap().borrow().left.clone();
        let z_right = found_node_ref.as_ref().unwrap().borrow().right.clone();
    
        let mut y = found_node_ref.clone();
        let mut y_original_color = y.as_ref().unwrap().borrow().color.clone();
    
        if z_left.is_none() {
            x = z_right.clone();
            self.transplant(&found_node_ref.clone(), &z_right.clone());
        } else if z_right.is_none() {
            x = z_left.clone();
            self.transplant(&found_node_ref.clone(), &x.clone());
        } else {
            let minimum_node = self.find_minimum(&found_node_ref.clone().as_ref().unwrap().borrow().right.clone());
            y_original_color = minimum_node.as_ref().unwrap().borrow().color.clone();
            x = minimum_node.as_ref().unwrap().borrow().right.clone();
            
            if let Some(minimum_parent) = minimum_node.as_ref().unwrap().borrow().parent.clone() {
                if minimum_parent.upgrade().unwrap().borrow().key == found_node_ref.as_ref().unwrap().borrow().key {
                    // Here you have access to the key of the minimum node's parent
                    println!("Minimum parent key: {:?}", minimum_parent.upgrade().unwrap().borrow().key);
            
                    if let Some(x_ref) = x.as_ref() {
                        x_ref.borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
                    }
                }
            } else {
                self.transplant(&minimum_node.clone(), &minimum_node.as_ref().unwrap().borrow().right.clone());
                minimum_node.as_ref().unwrap().borrow_mut().right = z_right.clone();
                minimum_node.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
            }
            self.transplant(&found_node_ref.clone(), &minimum_node.clone());
            minimum_node.as_ref().unwrap().borrow_mut().left = z_left.clone();
            minimum_node.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
            minimum_node.as_ref().unwrap().borrow_mut().color = found_node_ref.as_ref().unwrap().borrow().color.clone();
        }
        println!("{}", Rc::strong_count(&found_node_ref.as_ref().unwrap()));
        if y_original_color == NodeColor::Black && x.is_some() {
            // self.fix_delete(x.as_ref().unwrap().clone());
        }
    }
    

    fn fix_delete(&mut self, x: Tree<T>) {
        let mut x_ref = x.clone();
    
        while x_ref.borrow().parent.is_some() && x_ref.borrow().color == NodeColor::Black {
            if Some(true) == self.is_left_child(&x_ref) {
                //find grand_parent
                let x_parent = match x_ref.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                    None => None,
                };

                //find sibling
                // if sibling exists it must be right child of parent
                let sibling_ref = match x_parent {
                    Some(ref parent_node) => {
                        parent_node.borrow().right.clone()
                    },
                    None => None,
                };

                if sibling_ref.is_some() && sibling_ref.as_ref().unwrap().borrow().color == NodeColor::Red {
                    let sibling = sibling_ref.as_ref().unwrap().clone();
                    sibling.borrow_mut().color = NodeColor::Black;
    

                    //set parent to red
                    x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;

                    //perform left rotation
                    self.rotate_left(x_parent.as_ref().unwrap().clone());

                    // Update sibling reference
                    if let Some(parent_weak) = &x_ref.borrow().parent {
                        if let Some(parent) = parent_weak.upgrade() {
                            let new_sibling_ref = x_ref.borrow().right.clone();
                            parent.borrow_mut().right = new_sibling_ref;
                        }
                    }
                } else if sibling_ref.as_ref().unwrap().borrow().left.is_some() &&sibling_ref.as_ref().unwrap().borrow().right.is_some() {
                    if sibling_ref.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black && sibling_ref.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black {
                        let sibling = sibling_ref.as_ref().unwrap().clone();
                        sibling.borrow_mut().color = NodeColor::Red;

                        // TODO set x = x.parent
                        x_ref = x_parent.unwrap();
                    } else {
                        if sibling_ref.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black {
                            //w.left.color = black
                            //w .color = red
                            //right_rotate()
                            //w = x.parent.right
                            let w = sibling_ref.as_ref().unwrap().clone();
                            w.borrow_mut().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            w.borrow_mut().color = NodeColor::Red;
        
                            self.rotate_right(w.clone());
        
                            // Update sibling reference
                            if let Some(parent_weak) = &x_ref.borrow().parent {
                                if let Some(parent) = parent_weak.upgrade() {
                                    let new_sibling_ref = x_ref.borrow().right.clone();
                                    parent.borrow_mut().right = new_sibling_ref;
                                }
                            }
                            // Reassign sibling_ref to x.parent.right after rotation
                            let sibling_ref = x_ref.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().right.clone();
                          
                        }

                        // Case 4: Adjust colors and perform a left rotation
                        let w = sibling_ref.as_ref().unwrap().clone();
                        let mut new_x_ref = None;
                        // Case 4: Adjust colors and perform a left rotation
                        if let Some(parent_weak) = &x_ref.borrow().parent {
                            if let Some(parent) = parent_weak.upgrade() {
                                let w_color = &w.borrow().color;
                                w.borrow_mut().color = parent.borrow().color.clone();
                                parent.borrow_mut().color = NodeColor::Black;
                                w.borrow_mut().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                self.rotate_left(parent.clone());

                                if let Some(root) = &self.root {
                                    new_x_ref = Some(root.clone());
                                }
                                
                            }
         
                        }
                        if let Some(new_ref) = new_x_ref {
                            x_ref = new_ref;
                        }


                    }
                    
                }

            } else {
                //find grand_parent
                let x_parent = match x_ref.borrow().parent {
                    Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                    None => None,
                };

                //find sibling
                // if sibling exists it must be right child of parent
                let sibling_ref = match x_parent {
                    Some(ref parent_node) => {
                        parent_node.borrow().left.clone()
                    },
                    None => None,
                };

                if sibling_ref.is_some() && sibling_ref.as_ref().unwrap().borrow().color == NodeColor::Red {
                    let sibling = sibling_ref.as_ref().unwrap().clone();
                    sibling.borrow_mut().color = NodeColor::Black;
    

                    //set parent to red
                    x_parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;

                    //perform left rotation
                    self.rotate_right(x_parent.as_ref().unwrap().clone());

                    // Update sibling reference
                    if let Some(parent_weak) = &x_ref.borrow().parent {
                        if let Some(parent) = parent_weak.upgrade() {
                            let new_sibling_ref = x_ref.borrow().right.clone();
                            parent.borrow_mut().right = new_sibling_ref;
                        }
                    }
                } else if sibling_ref.as_ref().unwrap().borrow().right.is_some() &&sibling_ref.as_ref().unwrap().borrow().left.is_some() {
                    if sibling_ref.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black && sibling_ref.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black {
                        let sibling = sibling_ref.as_ref().unwrap().clone();
                        sibling.borrow_mut().color = NodeColor::Red;

                        // TODO set x = x.parent
                        x_ref = x_parent.unwrap();
                    } else {
                        if sibling_ref.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black {
                            //w.left.color = black
                            //w .color = red
                            //right_rotate()
                            //w = x.parent.right
                            let w = sibling_ref.as_ref().unwrap().clone();
                            w.borrow_mut().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            w.borrow_mut().color = NodeColor::Red;
        
                            self.rotate_left(w.clone());
        
                            // Update sibling reference
                            if let Some(parent_weak) = &x_ref.borrow().parent {
                                if let Some(parent) = parent_weak.upgrade() {
                                    let new_sibling_ref = x_ref.borrow().left.clone();
                                    parent.borrow_mut().right = new_sibling_ref;
                                }
                            }
                            // Reassign sibling_ref to x.parent.right after rotation
                            let sibling_ref = x_ref.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().left.clone();
                          
                        }

                        // Case 4: Adjust colors and perform a left rotation
                        let w = sibling_ref.as_ref().unwrap().clone();
                        let mut new_x_ref = None;
                        // Case 4: Adjust colors and perform a left rotation
                        if let Some(parent_weak) = &x_ref.borrow().parent {
                            if let Some(parent) = parent_weak.upgrade() {
                                let w_color = &w.borrow().color;
                                w.borrow_mut().color = parent.borrow().color.clone();
                                parent.borrow_mut().color = NodeColor::Black;
                                w.borrow_mut().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                self.rotate_right(parent.clone());

                                if let Some(root) = &self.root {
                                    new_x_ref = Some(root.clone());
                                }
                                
                            }
         
                        }
                        if let Some(new_ref) = new_x_ref {
                            x_ref = new_ref;
                        }


                    }
                    
                }
            }
        }

        x.borrow_mut().color = NodeColor::Black;
    }


    fn transplant(&mut self, u: &RedBlackTree<T>, v: &RedBlackTree<T>) {
        let u_node = u.as_ref().unwrap().clone();
        let u_parent = u.as_ref().unwrap().borrow().parent.clone();
    
        // Check if u is root
        if u_parent.is_none() {
            self.root = v.clone();
        } else if Some(true) == self.is_left_child(&u_node) {
            if let Some(u_parent_ref) = u_parent.clone() {
                let u_parent_upgrade = u_parent_ref.upgrade().unwrap();
                u_parent_upgrade.borrow_mut().left = v.clone();
            }
        } else {
            if let Some(u_parent_ref) = u_parent.clone() {
                let u_parent_upgrade = u_parent_ref.upgrade().unwrap();
                u_parent_upgrade.borrow_mut().right = v.clone();
            }
        }
    
        // Set the parent of v to be the parent of u
        if let Some(v_node) = v.clone() {
            if let Some(u_parent_ref) = u_parent.clone() {
                let u_parent_upgrade = u_parent_ref.upgrade().unwrap();
                v_node.borrow_mut().parent = Some(Rc::downgrade(&u_parent_upgrade));

            } else {
                v_node.borrow_mut().parent = None;
            }
        }
    }
    

    pub fn find_minimum(&mut self, tree: &RedBlackTree<T>) -> RedBlackTree<T> {
        let root = self.root.clone();
        self.find_minimum_recursion(tree.clone())
    }
    
    pub fn find_minimum_recursion(&mut self, tree: RedBlackTree<T>) -> RedBlackTree<T> {
        match tree {
            Some(node) => {
                if node.borrow().left.is_none() {
                    println!("Minimum on right is: {:?}", node.borrow().key);
                    Some(node.clone())
                } else {
                    self.find_minimum_recursion(node.borrow().left.clone())
                }
            },
            None => None,
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
        println!("{}{}{:?} ({:?}), RC-count = {}", prefix, if is_left { "L├── " } else { "R└── " }, node_borrowed.key, node_borrowed.color,Rc::strong_count(&node));

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

}

