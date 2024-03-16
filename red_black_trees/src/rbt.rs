use std::cell::RefCell;
use std::rc::Rc;
use std::{clone, fmt};
use std::fmt::Debug;
use std::fmt::Display;

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
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedBlackTreeSet<T: Ord+Display+Debug+Copy> {
    pub root: RedBlackTree<T>,
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self {
        RedBlackTreeSet { root: None }
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
            // self.fix_insert(new_node.clone());
            // TODO: Perform any necessary balancing
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
        println!("inserted Node: {:?}, colour orginally: {:?}", new_node.borrow().key, new_node.borrow().color);
        if is_root{
            return
        } else {
            let mut node_ref = new_node.borrow().clone();

            while new_node.borrow().parent.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
                // Find the parent node
                // it is safe to use unwrap here since we already verified the parent has a value
                let parent = node_ref.parent.clone().unwrap();
                
                //parent is a left child
                if Some(true) == self.is_left_child(&parent) {
    
                    //find grand_parent
                    let grandparent = match parent.borrow().parent {
                        Some(ref parent_node) => {
                            println!("\tThe orginal parent colour is: {:?}", parent_node.borrow().color);
                            Some(parent_node.clone())
                        },
                        None => None,
                    };
    
                    //find uncle
                    // if uncle exists it must be right child of gp
                    let uncle = match grandparent {
                        Some(ref grandparent_node) => {
                            grandparent_node.borrow().right.clone()
                        },
                        None => None,
                    };

                    if let Some(ref uncle) = uncle {
                        // case 1: uncle is red
                        // we flip colour of all other nodes
                        if uncle.borrow().color == NodeColor::Red {
                            //flip uncle colour
                            let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                            uncle.borrow_mut().color = new_uncle_colour;
                            println!("\tThe uncle is: {:?} and new color is: {:?}", uncle.borrow().key, uncle.borrow().color);
            
                            // Flip parent color
                            let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                            parent.borrow_mut().color = new_parent_colour;
                            println!("\tThe parent is: {:?} and new color is: {:?}", parent.borrow().key, parent.borrow().color);
        
                            // Flip grandparent color
                            if let Some(ref grandparent) = grandparent {
                                let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                                grandparent.borrow_mut().color = new_grandparent_colour;
                                println!("\tThe grandparent is: {:?} and new color is: {:?}", grandparent.borrow().key, grandparent.borrow().color);
                            } else {
                                println!("\tNo grandparent");
                            }
                        
                        } else {
                            // new node is a right child
                            if Some(false) == self.is_left_child(&new_node) {
                                // TODO figure out if this is needed?
                                // z = z.parent //marked z.parent as new z
                                self.rotate_left(parent)
                            
                            }
                        }
                        
                    } else {
                        println!("\tNo uncle");
                    }
                } else { // the parent of new_node is right
                        //find grand_parent

                        let grandparent = match parent.borrow().parent {
                        Some(ref parent_node) => {
                            println!("\tThe orginal parent colour is: {:?}", parent_node.borrow().color);
                            Some(parent_node.clone())
                        },
                        None => None,
                    };

                    //find uncle
                    // if uncle exists it must be left child of gp
                    let uncle = match grandparent {
                        Some(ref grandparent_node) => {
                            grandparent_node.borrow().left.clone()
                        },
                        None => None,
                    };

                    if let Some(ref uncle) = uncle {
                        // case 1: uncle is red
                        // we flip colour of all other nodes
                        if uncle.borrow().color == NodeColor::Red {
                            //flip uncle colour
                            let new_uncle_colour = NodeColor::flip_color(uncle.borrow().clone().color);
                            uncle.borrow_mut().color = new_uncle_colour;
                            println!("\tThe uncle is: {:?} and new color is: {:?}", uncle.borrow().key, uncle.borrow().color);
            
                            // Flip parent color
                            let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                            parent.borrow_mut().color = new_parent_colour;
                            println!("\tThe parent is: {:?} and new color is: {:?}", parent.borrow().key, parent.borrow().color);
        
                            // Flip grandparent color
                            if let Some(ref grandparent) = grandparent {
                                let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                                grandparent.borrow_mut().color = new_grandparent_colour;
                                println!("\tThe grandparent is: {:?} and new color is: {:?}", grandparent.borrow().key, grandparent.borrow().color);
                            } else {
                                println!("\tNo grandparent");
                            }
                        
                        } else {
                            // new node is a left child
                            if Some(true) == self.is_left_child(&new_node) {
                                // TODO figure out if this is needed?
                                // z = z.parent //marked z.parent as new z
                                self.rotate_right(parent)
                            
                            }
                        }
                        
                    } else {
                        println!("\tNo uncle");
                    }
                }
    
            }
        }

        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;

    }
    
    
    fn is_left_child(&self, node: &Tree<T>) -> Option<bool> {
        if let Some(ref parent) = node.borrow().parent.clone() {
            if let Some(ref left_child) = parent.borrow().left.clone() {
                if Rc::ptr_eq(left_child, node) {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn rotate_left(&mut self, x: Tree<T>) {
        // following popular pseudo code found online and converting to rust
        let mut x_borrowed = x.borrow_mut();

        //y = x.right
        let y = x_borrowed.right.as_ref().expect("Cannot left rotate with no right child").clone();

        //x.right = y.left
        x_borrowed.right = y.borrow_mut().left.clone();
        
        // if y.left != null
        if let Some(ref y_left) = y.borrow_mut().left {
            //y.left.p = x
            y_left.borrow_mut().parent = Some(x.clone());
        }

        //y.p = x.p
        y.borrow_mut().parent = x_borrowed.parent.clone();
        
        // if x.p != null
        if x_borrowed.parent.is_none() {
            //root = y
            self.root = Some(y.clone());
        // x == x.p.left
        } else if x == x_borrowed.parent.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().clone() {
            // x.p.left = y
            x_borrowed.parent.as_ref().unwrap().borrow_mut().left = Some(y.clone());
        } else {
            //x.p.right = y
            x_borrowed.parent.as_ref().unwrap().borrow_mut().right = Some(y.clone());
        }
        //y.left = x
        y.borrow_mut().left = Some(x.clone());
        //x.p = y
        x_borrowed.parent = Some(y.clone());
    }

    pub fn rotate_right(&mut self, x: Tree<T>) {
        // following popular pseudo code found online and converting to rust
        let mut x_borrowed = x.borrow_mut();

        let y = x_borrowed.left.as_ref().expect("Cannot left rotate with no right child").clone();

        x_borrowed.left = y.borrow_mut().right.clone();
        
        if let Some(ref y_right) = y.borrow_mut().right {
            y_right.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x_borrowed.parent.clone();
        
        if x_borrowed.parent.is_none() {
            self.root = Some(y.clone());
   
        } else if x == x_borrowed.parent.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().clone() {
            x_borrowed.parent.as_ref().unwrap().borrow_mut().left = Some(y.clone());
        } else {
            x_borrowed.parent.as_ref().unwrap().borrow_mut().right = Some(y.clone());
        }

        y.borrow_mut().right = Some(x.clone());
        x_borrowed.parent = Some(y.clone());
    }

   

    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            self.print_recursive(root.clone());
        } else {
            println!("Empty tree");
        }
    }

    fn print_recursive(&self, node: Tree<T>) {
        let node_borrowed = node.borrow_mut();

        println!("Key: {:?}, Colour: {:?}", node_borrowed.key, node_borrowed.color);
      

        if let Some(left) = &node_borrowed.left {
            self.print_recursive(left.clone());
        }

        if let Some(right) = &node_borrowed.right {
            self.print_recursive(right.clone());
        }
    }

}

