use std::cell::RefCell;
use std::iter::Successors;
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
    pub left: RedBlackTree<T>,
    pub right: RedBlackTree<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedBlackTreeSet<T: Ord+Display+Debug+Copy> {
    pub root: RedBlackTree<T>,
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self {
        RedBlackTreeSet { root: None }
    }

    pub fn find(&mut self, key: T) -> RedBlackTree<T>{
        let root = self.root.clone();
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
        // if is_root{
        //     return
        // } else {
        let mut parent_color = new_node.borrow().parent.as_ref().unwrap().borrow().clone().color;
        let mut node_ref = new_node.borrow().clone();

        while node_ref.clone().parent.is_some() && node_ref.parent.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
            
            let node = new_node.clone();
            
            // Find the parent node
            // it is safe to use unwrap here since we already verified the parent has a value
            let mut parent = node_ref.parent.clone().unwrap();
            
            //parent is a left child
            if self.is_left_child(&parent).is_some() {
                
                //find grand_parent
                let grandparent = match parent.borrow().parent {
                    Some(ref parent_node) => {
                        Some(parent_node.clone())
                    },
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
                    }
                
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                        }
                        
                        node_ref = grandparent.borrow().clone();
                    } else {
                        println!("\tNo grandparent");
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
                    
                    println!("\tThe parent is: {:?} and new color is: {:?}", parent.borrow().key, parent.borrow().color);

                    // set grandparent color to black
                    // since we may have reassigned parent we should reget the grandparent

                    let grandparent_clone = match parent.borrow().parent {
                        Some(ref parent_node) => {
                            println!("\tThe orginal parent colour is: {:?}", parent_node.borrow().color);
                            Some(parent_node.clone())
                        },
                        None => None,
                    };



                    if let Some(ref grandparent) = grandparent_clone {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_right(grandparent.clone());
                        println!("\tThe grandparent is: {:?} and new color is: {:?}", grandparent.borrow().key, grandparent.borrow().color);
                    } else {
                        println!("\tNo grandparent");
                    }
                    
                }
                    
            } else {
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
                    }
                    
                    println!("\tThe uncle is: {:?} and new color is: {:?}", uncle.borrow().key, uncle.borrow().color);
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;
                    println!("\tThe parent is: {:?} and new color is: {:?}", parent.borrow().key, parent.borrow().color);

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
                        }
                        
                        println!("\tThe grandparent is: {:?} and new color is: {:?}", grandparent.borrow().key, grandparent.borrow().color);
                        node_ref = grandparent.borrow().clone();
                    } else {
                        println!("\tNo grandparent");
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
                    println!("\tThe parent is: {:?} and new color is: {:?}", parent.borrow().key, parent.borrow().color);

                    // since we may have reassigned parent we should reget the grandparent

                    let grandparent_clone = match parent.borrow().parent {
                        Some(ref parent_node) => {
                            println!("\tThe orginal parent colour is: {:?}", parent_node.borrow().color);
                            Some(parent_node.clone())
                        },
                        None => None,
                    };


                    if let Some(ref grandparent) = grandparent_clone {
                        grandparent.borrow_mut().color = NodeColor::Red;
                        self.rotate_left(grandparent.clone());
                        println!("\tThe grandparent is: {:?} and new color is: {:?}", grandparent.borrow().key, grandparent.borrow().color);
                    } else {
                        println!("\tNo grandparent");
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

    pub fn rotate_left(&mut self, tree_node: Tree<T>) {
        let cur_parent = tree_node;
        let right_child = cur_parent.borrow().right.clone();

        // take the left child of right child and make it the right child of the current parent
        cur_parent.borrow_mut().right = match right_child {
            Some(ref right_child) => {right_child.borrow().left.clone()},
            None => {None}
        };

        if right_child.is_some() {
            // make right child's parent the current grandparent
            right_child.as_ref().unwrap().borrow_mut().parent = cur_parent.borrow().parent.clone();
            if right_child.as_ref().unwrap().borrow().left.is_some() {
                // make right_child's left child's parent the current parent
                let l = right_child.as_ref().unwrap().borrow().left.clone();
                l.unwrap().borrow_mut().parent = Some(cur_parent.clone());
            }
        }

        match cur_parent.borrow().clone().parent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < cur_parent.borrow().clone().key {
                    grandparent.borrow_mut().right = right_child.clone();
                } else {
                    grandparent.borrow_mut().left = right_child.clone();
                }
            },
            None => {
                // grandparent is None, so make the right_child's parent None
                self.root = right_child.clone();
                // right_child.as_ref().unwrap().borrow_mut().parent = None;
            },
        }
        // make right_child's left child equal to the parent
        right_child.as_ref().unwrap().borrow_mut().left = Some(cur_parent.clone());
        // make parent's parent equal to right_child
        cur_parent.borrow_mut().parent = right_child.clone();
    }

    pub fn rotate_right(&mut self, tree_node: Tree<T>) {
        let cur_parent = tree_node;
        let left_child = cur_parent.borrow().left.clone();

        // take the right child of left_child and make it the left child of current parent
        cur_parent.borrow_mut().left = match left_child {
            Some(ref left_child) => {left_child.borrow().right.clone()},
            None => {None}
        };

        if left_child.is_some() {
            // make left child's parent the current grandparent
            left_child.as_ref().unwrap().borrow_mut().parent = cur_parent.borrow().parent.clone();
            if left_child.as_ref().unwrap().borrow().right.is_some() {
                // make left_child's right child's parent the current parent
                let r = left_child.as_ref().unwrap().borrow().right.clone();
                r.unwrap().borrow_mut().parent = Some(cur_parent.clone());
            }
        }

        match cur_parent.borrow().clone().parent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < cur_parent.borrow().clone().key {
                    grandparent.borrow_mut().right = left_child.clone();
                } else {
                    grandparent.borrow_mut().left = left_child.clone();
                }
            },
            None => {
                // grandparent is None, so make the left_child's parent None
                self.root = left_child.clone();
                // left_child.as_ref().unwrap().borrow_mut().parent = None;
            },
        }
        // make left_child's right child equal to the parent
        left_child.as_ref().unwrap().borrow_mut().right = Some(cur_parent.clone());
        // make parent's parent equal to left_child
        cur_parent.borrow_mut().parent = left_child.clone();
    }
    

    pub fn rotate_left_old(&mut self, x: Tree<T>) {
        // following popular pseudo code found online and converting to rust
        let mut x_borrowed = x.borrow_mut();


        //y = x.right
        let y = x_borrowed.right.as_ref().expect("Cannot left rotate with no right child").clone();

        //x.right = y.left
        x_borrowed.right = y.borrow().left.clone();
        
        // if y.left != null
        if let Some(ref y_left) = y.borrow().left {
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
        } else if x == x_borrowed.parent.as_ref().unwrap().borrow().left.as_ref().unwrap().clone() {
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

   pub fn rotate_right_old(&mut self, x: Tree<T>) {
        // following popular pseudo code found online and converting to rust
        let mut x_borrowed = x.borrow_mut();

        let y = x_borrowed.left.as_ref().expect("Cannot left rotate with no right child").clone();

        x_borrowed.left = y.borrow().right.clone();
        
        if let Some(ref y_right) = y.borrow().right {
            y_right.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x_borrowed.parent.clone();
        
        if x_borrowed.parent.is_none() {
            self.root = Some(y.clone());
   
        } else if x == x_borrowed.parent.as_ref().unwrap().borrow().right.as_ref().unwrap().clone() {
            x_borrowed.parent.as_ref().unwrap().borrow_mut().left = Some(y.clone());
        } else {
            x_borrowed.parent.as_ref().unwrap().borrow_mut().right = Some(y.clone());
        }

        y.borrow_mut().right = Some(x.clone());
        x_borrowed.parent = Some(y.clone());
    }

    pub fn delete(&mut self, key: T) {
        let found_node = self.find(key);
        let found_node_ref = found_node;

        let found = found_node_ref.as_ref().unwrap().borrow();
        let v = found_node_ref.as_ref().unwrap().borrow().left.clone();
        let w = found_node_ref.as_ref().unwrap().borrow().right.clone();
        
        let y = match found_node_ref.clone() {
            Some(node_ref) => Some(node_ref.clone()),
            None => { 
                print!("Key not found");
                return;
            }

        };

        let mut y_orginal_color = y.as_ref().unwrap().borrow().color.clone();
        let mut x:RedBlackTree<T>;

        if found_node_ref.as_ref().unwrap().borrow().left.is_none() {
            x = found_node_ref.as_ref().unwrap().borrow().right.clone();
            self.transplant(&found_node_ref.clone(), &x.clone())
        } else if  found_node_ref.as_ref().unwrap().borrow().right.is_none() {
            x = found_node_ref.as_ref().unwrap().borrow().left.clone();
            self.transplant(&found_node_ref.clone(), &x.clone())
        
        // node has 2 children
        } else {
        
            let y = self.find_minimum(&w.clone());

            y_orginal_color = y.as_ref().unwrap().borrow().color.clone();
            x = y.as_ref().unwrap().borrow().clone().right;
            if y.as_ref().unwrap().borrow().clone().parent.as_ref().unwrap().borrow().clone().key == found_node_ref.as_ref().unwrap().borrow().clone().key {
                if x.is_some() {
                    x.as_ref().unwrap().borrow_mut().parent = y.clone();
                }
            } else {
                self.transplant(&y.clone(), &y.as_ref().unwrap().borrow().right.clone());
                y.as_ref().unwrap().borrow_mut().right = found_node_ref.as_ref().unwrap().borrow().right.clone();
                y.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().parent = y.clone();
            }
            self.transplant(&found_node_ref.clone(), &y.clone());
            y.as_ref().unwrap().borrow_mut().left = v.clone();
            v.as_ref().unwrap().borrow_mut().parent = y.clone();
            y.as_ref().unwrap().borrow_mut().color = found_node_ref.as_ref().unwrap().borrow().color.clone();
        }
        if y_orginal_color == NodeColor::Black {
            self.fix_delete(x.clone());
        }
    }

    fn fix_delete(&mut self, x: RedBlackTree<T>) {
        // w = sibling of x
        while  x.as_ref().unwrap().borrow().parent.is_some() && x.as_ref().unwrap().borrow().color == NodeColor::Black {
            // x is the left child of parent
            let x_parent = x.as_ref().unwrap().borrow().parent.clone();
            if Some(true) == self.is_left_child( x.as_ref().unwrap()) {
                // if x has a sibling (w)
                if x_parent.as_ref().unwrap().borrow().right.is_some() {
                    let w = x_parent.as_ref().unwrap().borrow().right.clone();

                    //case 1 sibling is red
                    if w.as_ref().unwrap().borrow().color == NodeColor::Red {
                        w.as_ref().unwrap().borrow_mut().color = NodeColor::Black
                    }


                }
            }

        }
    }

    fn transplant(&mut self, u: &RedBlackTree<T>, v: &RedBlackTree<T>) {
        let u_node = u.as_ref().unwrap().clone();
        let u_parent = u.as_ref().unwrap().borrow().parent.clone();
        //check if u is root
        if u_parent.is_none(){
            self.root = v.clone();
        } else if Some(true) == self.is_left_child(&u_node.clone()) {
            u_parent.as_ref().unwrap().borrow_mut().left = v.clone();
        } else {
            u_parent.as_ref().unwrap().borrow_mut().right = v.clone();
        }
    }

    pub fn find_minimum(&mut self, tree: &RedBlackTree<T>) -> RedBlackTree<T>{
        let root = self.root.clone();
        self.find_minimum_recursion(&tree.clone())
    }

    pub fn find_minimum_recursion(&mut self, tree: &RedBlackTree<T>) -> RedBlackTree<T>{
        match tree {
            Some(node) => {
                if node.borrow().left.is_none() {
                    println!("minimum on right is: {:?}", node.borrow().key);
                    Some(node.clone())
                } else {
                    self.find_minimum_recursion(&node.borrow().left.clone())
                }
            },
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
        println!("{}{}{:?} ({:?})", prefix, if is_left { "L├── " } else { "R└── " }, node_borrowed.key, node_borrowed.color);

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

