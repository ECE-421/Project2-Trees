use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
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

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RedBlackTreeSet<T> {
    pub root: RedBlackTree<T>,
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> {
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
            self.fix_insert(new_node);
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

    fn fix_insert(&mut self, new_node: Tree<T>) {
       
        let node = new_node.borrow_mut();
    
        if let Some(parent) = &node.parent {
            let grandparent = match parent.borrow().parent.clone() {
                Some(grandparent) => grandparent,
                None => {
                    print!("None");
                    return; // Exit the function when there is no grandparent
                }
            };
    
            println!("\n\nGrandparent: {:?}", grandparent.borrow().key);
        }
        // case 0: node is root
        // what to do: Colour node black


        // case 1: nodes uncle is red
        // what to do: 
        // #1 verify node.parent.parent.(right/left).colour = red
        // #2 recolour node.parent, node.parent.parent and node.parent.parent.(right/left)

        // case 2: nodes uncle is black (triangle)
        // what to do: 
        // #1 verify node.parent.parent.(right/left).colour = black
        // #2 verify node is left child and node.parent is right child
        // #3 verify node is right child and node.parent is left child
        // if #2 rotate right on node.parent
        // if #3 rotate left on node.parent

        // case 3: nodes uncle is black (line)
        // what to do: 
        // #1 verify node.parent.parent.(right/left).colour = black
        // #2 verify node is right child and node.parent is right child
        // #3 verify node is left child and node.parent is left child

        // if #2 rotate left on node.parent.parent
        // if #3 rotate right on node.parent.parent
        
    }

    pub fn left_rotate(&mut self, x: Tree<T>) {
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

    pub fn right_rotate(&mut self, x: Tree<T>) {
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

        println!("{:?}", node_borrowed.key);

        if let Some(left) = &node_borrowed.left {
            self.print_recursive(left.clone());
        }

        if let Some(right) = &node_borrowed.right {
            self.print_recursive(right.clone());
        }
    }
   // TODO: Implement balancing functions (left and right rotations, recoloring) here
}

