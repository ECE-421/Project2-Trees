use std::cell::RefCell;
use std::iter::Successors;
use std::rc::{Rc, Weak};
use std::{clone, fmt};
use std::fmt::Debug;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};


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

impl<T: Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let parent_key = match &self.parent {
            Some(weak_parent) => {
                if let Some(parent) = weak_parent.upgrade() {
                    parent.borrow().key.to_string()
                } else {
                    "None".to_string()
                }
            }
            None => "None".to_string(),
        };

        let left_key = match &self.left {
            Some(left_node) => left_node.borrow().key.to_string(),
            None => "None".to_string(),
        };

        let right_key = match &self.right {
            Some(right_node) => right_node.borrow().key.to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "Key: {}, Color: {:?}, Parent: {}, Left: {}, Right: {}",
            self.key, self.color, parent_key, left_key, right_key
        )
    }
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self {
        RedBlackTreeSet { root: None }
    }

    pub fn isEmpty(&self) -> bool {
        return self.root.is_none()
    }

    pub fn find(&mut self, key: T) -> RedBlackTree<T>{

        let root = self.root.clone();

        if self.isEmpty(){
            return None;
        }

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
                    let tnode = node.borrow().clone();
                    println!("Found Node: {}", tnode);
                    // println!("Found {}\n", node.borrow().key);
                    Some(node.clone())
                }
            },
            None => None,
        }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(key)));
        if self.find(key).is_none() {

            if let Some(mut root) = self.root.clone() {
                self.insert_recursive(&root, &new_node);
                self.fix(new_node.clone());
            } else {
                // If the tree is empty, make the new node the root and color it black
                new_node.borrow_mut().color = NodeColor::Black;
                self.root = Some(new_node);
            
            }
        } else {
            println!("{:?} is a duplicate", key);
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
        // println!("Inserted Node: {:?}", new_node.borrow().key);


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
        
            let parent = node_ref.borrow().parent.clone().unwrap().upgrade().unwrap().clone();
            
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
                    }
                
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;

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

                        self.rotate_left(parent_upgrade);
                    }

                    let parent_weak = node_ref.borrow().parent.clone().unwrap();
                    let parent_upgrade = parent_weak.upgrade().unwrap();

                    parent_upgrade.borrow_mut().color = NodeColor::Black;
                  

                    // Case 3
                    let grandparent = match parent_upgrade.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent {
                        grandparent.borrow_mut().color = NodeColor::Red;
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
                    }
                
    
                    // Flip parent color
                    let new_parent_colour = NodeColor::flip_color(parent.borrow().clone().color);
                    parent.borrow_mut().color = new_parent_colour;

                    // Flip grandparent color
                    if let Some(ref grandparent) = grandparent {
                        let new_grandparent_colour = NodeColor::flip_color(grandparent.borrow().clone().color);
                        {
                            grandparent.borrow_mut().color = new_grandparent_colour;
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
                        self.rotate_right(parent_upgrade);
                    }

                    // Case 3
                    let parent_weak = node_ref.borrow().parent.clone().unwrap();
                    let parent_upgrade = parent_weak.upgrade().unwrap();
                    
                    parent_upgrade.borrow_mut().color = NodeColor::Black;
                   

                    let grandparent = match parent_upgrade.borrow().parent {
                        Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                        None => None,
                    };

                    if let Some(ref grandparent) = grandparent {
                        grandparent.borrow_mut().color = NodeColor::Red;
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

        let mut parent = None;
        
        println!("{}", Rc::strong_count(&found_node_ref.as_ref().unwrap()));
        println!("Found node key: {:?}", found_node_ref.as_ref().unwrap().borrow().key);

        // let found = found_node_ref.as_ref().unwrap().borrow();
        let z_parent = found_node_ref.as_ref().unwrap().borrow().parent.clone();
        let z_left = &found_node_ref.as_ref().unwrap().borrow().left;
        let z_right = &found_node_ref.as_ref().unwrap().borrow().right;
    
        let mut y = found_node_ref.clone();
        let mut y_original_color = y.as_ref().unwrap().borrow().color.clone();
    
        if z_left.is_none() {
            x = z_right.clone();
            self.transplant(&found_node_ref.clone(), &z_right.clone());
        } else if z_right.is_none() {
            x = z_left.clone();
            self.transplant(&found_node_ref.clone(), &x.clone());
        } else {
            let y = self.find_minimum(&found_node_ref.clone().as_ref().unwrap().borrow().right.clone()).clone();
            println!("the minimum node was: {}", y.as_ref().unwrap().borrow().key);
            y_original_color = y.as_ref().unwrap().borrow().color.clone();
            x = y.as_ref().unwrap().borrow().right.clone();


            let minimum_parent = match y.as_ref().unwrap().borrow().parent {
                Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                None => None,
            };

            println!("sending this parent:{:}",minimum_parent.as_ref().unwrap().borrow().key);
            self.find(minimum_parent.as_ref().unwrap().borrow().key);
            parent = Some(Rc::downgrade(&minimum_parent.as_ref().unwrap()));
            self.find(minimum_parent.as_ref().unwrap().borrow().key);
            if minimum_parent.unwrap().borrow().key == found_node_ref.as_ref().unwrap().borrow().key {
                // Here you have access to the key of the minimum node's parent
                // println!("Minimum parent key: {:?}", minimum_parent.upgrade().unwrap().borrow().key);
        
                if let Some(x_ref) = x.as_ref() {
                    x_ref.borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
                }
            } else {
                self.transplant(&y.clone(), &y.as_ref().unwrap().borrow().right.clone());
               
                y.as_ref().unwrap().borrow_mut().right = z_right.clone();
                y.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
                let minimum_parent = match y.as_ref().unwrap().borrow().parent {
                    Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                    None => None,
                };
                println!("bewfore transpose");
                self.find(minimum_parent.as_ref().unwrap().borrow().key);
            }

            let minimum_parent = match y.as_ref().unwrap().borrow().parent {
                Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                None => None,
            };
            println!("after transpose fouynd node: {}", found_node_ref.as_ref().unwrap().borrow().key);
            self.find(minimum_parent.as_ref().unwrap().borrow().key);
            self.transplant(&found_node_ref.clone(), &y.clone());
            let minimum_parent = match y.as_ref().unwrap().borrow().parent {
                Some(ref parent_node) => Some(parent_node.upgrade().unwrap()),
                None => None,
            };
            println!("after transpose");
            println!("after transpose fouynd node: {}", found_node_ref.as_ref().unwrap().borrow().key);
            self.find(minimum_parent.as_ref().unwrap().borrow().key);
            self.find(found_node_ref.as_ref().unwrap().borrow().key);


            println!("after transpose fouynd node: {}", z_left.as_ref().unwrap().borrow().key);
            println!("after transpose fouynd node: {}", y.as_ref().unwrap().borrow().key);
            // println!("after transpose fouynd node: {}", z_left.as_ref().unwrap().borrow().key);
            
            y.as_ref().unwrap().borrow_mut().left = z_left.clone();
            y.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
            y.as_ref().unwrap().borrow_mut().color = found_node_ref.as_ref().unwrap().borrow().color.clone();
        }
        println!("{}", Rc::strong_count(&found_node_ref.as_ref().unwrap()));
        self.print_tree();
        if y_original_color == NodeColor::Black {
            self.fix_delete(x, parent);
        }
    }
    

    fn fix_delete(&mut self, x: RedBlackTree<T>, parent: Parent<T>) {
        self.print_tree();
        let mut x_ref = x.clone();
        let parent_ref_weak = parent.clone();
        // let mut parent_ref = parent_ref_weak.unwrap().upgrade();


    
        // while parent_ref_weak.as_ref().unwrap().upgrade().as_ref().unwrap().borrow().parent.is_some() && (x_ref.is_none() || x_ref.as_ref().unwrap().borrow().color == NodeColor::Black) {
            
            // cant use is left function since x may be null
            if x_ref.is_none() || (x_ref.as_ref().unwrap().borrow().key == parent.as_ref().unwrap().upgrade().unwrap().borrow().left.as_ref().unwrap().borrow().key) {
                // right sibling exists and is red

                let mut right_sibling = match parent {
                    Some(ref parent_node) => {
                        println!("The key for right sibling is: {}", parent_node.upgrade().as_ref().unwrap().borrow().key);
                        Some(parent_node.upgrade().as_ref().unwrap().borrow().right.as_ref().unwrap().clone())},
                    None => None,
                };

                // case 1 sibling is red
                if right_sibling.is_some() && right_sibling.as_ref().unwrap().borrow().color == NodeColor::Red {
                    let mut w = right_sibling.as_ref().unwrap().clone();
                    w.borrow_mut().color = NodeColor::Black;
                    let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                    
                    self.rotate_left(parent_upgrade.as_ref().unwrap().clone());
                    parent_upgrade.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                    println!("sibling was: {}", w.borrow().key);
                    w = parent_upgrade.as_ref().expect("").borrow().right.as_ref().unwrap().clone();    
                    println!("sibling is now: {}", w.borrow().key);
                }

                
                 
                // case 2: Both children of the sibling are black
                if let Some(ref right_sibling_node) = right_sibling {
                    if (right_sibling_node.borrow().right.is_none() || right_sibling_node.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black) &&
                    (right_sibling_node.borrow().left.is_none() || right_sibling_node.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black) {
                        let mut w = right_sibling_node.clone();
                        w.borrow_mut().color = NodeColor::Red;
                        println!("parent was: {}", x_ref.as_ref().unwrap().borrow().key);
                        let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                        x_ref = parent_upgrade.clone();
                        println!("parent is now: {}", x_ref.as_ref().unwrap().borrow().key);
                    } else {
                        // Case 3: Sibling's right child is black
                        if let Some(ref w_node) = right_sibling {
                            if w_node.borrow().right.is_some() && w_node.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black {
                                w_node.borrow_mut().left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                w_node.borrow_mut().color = NodeColor::Red;
                                self.rotate_right(w_node.clone());
                                println!("sibling was: {}", right_sibling.as_ref().unwrap().borrow().key);
                                let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                                right_sibling = parent_upgrade.as_ref().unwrap().borrow().right.clone(); // Update w after rotation
                                println!("sibling is now: {}", right_sibling.as_ref().unwrap().borrow().key);
                            }
                        }

                        // Case 4: Sibling's right child is red
                        if let Some(ref w_node) = right_sibling {
                            let parent_upgrde = parent_ref_weak.as_ref().unwrap().upgrade().clone();
                            right_sibling.as_ref().unwrap().borrow_mut().color = parent_upgrde.as_ref().unwrap().borrow().color.clone();
                            parent_upgrde.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            right_sibling.as_ref().unwrap().borrow_mut().right.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                            self.rotate_left(parent_upgrde.as_ref().unwrap().clone());
                            
                            // x_ref = self.root.clone();

                        }
                    }
                }
            

     
            } else {
                // right sibling exists and is red
                let mut right_sibling = match parent {
                    Some(ref parent_node) => {
                        println!("The key for right sibling is: {}", parent_node.upgrade().as_ref().unwrap().borrow().key);
                        Some(parent_node.upgrade().as_ref().unwrap().borrow().right.as_ref().unwrap().clone())},
                    None => None,
                };

                // case 1 sibling is red
                if right_sibling.is_some() && right_sibling.as_ref().unwrap().borrow().color == NodeColor::Red {
                    let mut w = right_sibling.as_ref().unwrap().clone();
                    w.borrow_mut().color = NodeColor::Black;
                    let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                    
                    self.rotate_left(parent_upgrade.as_ref().unwrap().clone());
                    parent_upgrade.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                    println!("sibling was: {}", w.borrow().key);
                    w = parent_upgrade.as_ref().expect("").borrow().right.as_ref().unwrap().clone();    
                    println!("sibling is now: {}", w.borrow().key);
                }

                
                
                // case 2: Both children of the sibling are black
                if let Some(ref right_sibling_node) = right_sibling {
                    if (right_sibling_node.borrow().right.is_none() || right_sibling_node.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black) &&
                    (right_sibling_node.borrow().left.is_none() || right_sibling_node.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black) {
                        let mut w = right_sibling_node.clone();
                        w.borrow_mut().color = NodeColor::Red;
                        println!("parent was: {}", x_ref.as_ref().unwrap().borrow().key);
                        let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                        x_ref = parent_upgrade.clone();
                        println!("parent is now: {}", x_ref.as_ref().unwrap().borrow().key);
                    } else {
                        // Case 3: Sibling's right child is black
                        if let Some(ref w_node) = right_sibling {
                            if w_node.borrow().right.is_some() && w_node.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black {
                                w_node.borrow_mut().left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                w_node.borrow_mut().color = NodeColor::Red;
                                self.rotate_right(w_node.clone());
                                println!("sibling was: {}", right_sibling.as_ref().unwrap().borrow().key);
                                let parent_upgrade = parent_ref_weak.clone().unwrap().upgrade().clone();

                                right_sibling = parent_upgrade.as_ref().unwrap().borrow().right.clone(); // Update w after rotation
                                println!("sibling is now: {}", right_sibling.as_ref().unwrap().borrow().key);
                            }
                        }

                        // Case 4: Sibling's right child is red
                        if let Some(ref w_node) = right_sibling {
                            let parent_upgrde = parent_ref_weak.as_ref().unwrap().upgrade().clone();
                            right_sibling.as_ref().unwrap().borrow_mut().color = parent_upgrde.as_ref().unwrap().borrow().color.clone();
                            parent_upgrde.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            right_sibling.as_ref().unwrap().borrow_mut().right.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                            self.rotate_left(parent_upgrde.as_ref().unwrap().clone());
                            
                            // x_ref = self.root.clone();

                        }
                    }
                }
            }
        // }   
        if x_ref.is_some(){
            x_ref.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
        }
        
    }
    


    fn transplant(&mut self, u: &RedBlackTree<T>, v: &RedBlackTree<T>) {
        let u_node = u.as_ref().unwrap().clone();
        let u_parent = u.as_ref().unwrap().borrow().parent.clone();
            
        
        let u_parent_upgrade = match u_parent.clone() {
            Some(upgrade_parent) => upgrade_parent.upgrade().unwrap(),
            None => return, // Handle the case when u_parent is None, if needed
        };
        
    
        // Check if u is root
        if u_parent.is_none() {
            self.root = v.clone();
        } 

        
        if Some(true) == self.is_left_child(&u_node) {
            u_parent_upgrade.borrow_mut().left = v.clone();
        } else {
            u_parent_upgrade.borrow_mut().right = v.clone();
        }
    
        // Set the parent of v to be the parent of u
        if let Some(v_node) = v.clone() {
                println!("assgning v.parent to {}", u_parent_upgrade.borrow().key);
                v_node.borrow_mut().parent = Some(Rc::downgrade(&u_parent_upgrade.clone()));

        
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
                    let tnode = node.borrow().clone();
                    println!("is min:\n\n : {}", tnode);
                    Some(node.clone())
                } else {
                    let tnode = node.borrow().clone();
                    println!("Node from min:\n\n : {}", tnode);
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

