use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct RedBlackTreeSet<T> {
    root: RedBlackTree<T>,
}

impl<T: Ord + fmt::Debug> RedBlackTreeSet<T> {
    fn new() -> Self {
        RedBlackTreeSet { root: None }
    }

    fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red, // New nodes are always red initially
            key,
            parent: None,
            left: None,
            right: None,
        }));

        if let Some(root) = &self.root {
            self.insert_recursive(root.clone(), new_node.clone());
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

    fn left_rotate(&mut self, x: Tree<T>) {
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

    fn right_rotate(&mut self, x: Tree<T>) {
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

    fn print_tree(&self) {
        if let Some(root) = &self.root {
            self.print_recursive(root.clone());
        } else {
            println!("Empty tree");
        }
    }

    fn print_recursive(&self, node: Tree<T>) {
        let node_borrowed = node.borrow();

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



fn main() {
    // let mut rb_tree = RedBlackTreeSet::new();
    // rb_tree.insert(5);
    // rb_tree.insert(2);
    // rb_tree.insert(10);
    // rb_tree.insert(8);
    // rb_tree.insert(6);
    // rb_tree.insert(9);
    // rb_tree.insert(12);

    // // Print the tree before right rotation
    // println!("Before Right Rotation:");
    // rb_tree.print_tree();

    // // Find the node you want to rotate (for example, the root in this case)
    // let root_node = rb_tree.root.as_ref().unwrap().clone();

    // // Perform right rotation
    // rb_tree.left_rotate(root_node);

    // // Print the tree after right rotation
    // println!("After Right Rotation:");


    // Create a Red-Black Tree manually
    let node_2 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 2,
        parent: None,
        left: None,
        right: None,
    }));

    let node_6 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 6,
        parent: None,
        left: None,
        right: None,
    }));

    let node_9 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 9,
        parent: None,
        left: None,
        right: None,
    }));

    let node_8 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 8,
        parent: None,
        left: Some(Rc::clone(&node_6)),
        right: Some(Rc::clone(&node_9)),
    }));

    let node_12 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 12,
        parent: None,
        left: None,
        right: None,
    }));

    let node_10 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 10,
        parent: None,
        left: Some(Rc::clone(&node_8)),
        right: Some(Rc::clone(&node_12)),
    }));

    let node_5 = Rc::new(RefCell::new(TreeNode {
        color: NodeColor::Black,
        key: 5,
        parent: None,
        left: Some(Rc::clone(&node_2)),
        right: Some(Rc::clone(&node_10)),
    }));

    //assign parents:
    node_2.borrow_mut().parent = Some(Rc::clone(&node_5));
    node_10.borrow_mut().parent = Some(Rc::clone(&node_5));

    node_8.borrow_mut().parent = Some(Rc::clone(&node_10));
    node_12.borrow_mut().parent = Some(Rc::clone(&node_10));

    node_6.borrow_mut().parent = Some(Rc::clone(&node_8));
    node_9.borrow_mut().parent = Some(Rc::clone(&node_8));

    let mut rb_tree = RedBlackTreeSet::new();
    rb_tree.root = Some(Rc::clone(&node_5));
    rb_tree.print_tree();

    println!("\n");
    // Create a Red-Black Tree using your insert method
    let mut inserted_tree = RedBlackTreeSet::new();
    inserted_tree.insert(10);
    inserted_tree.insert(5);
    inserted_tree.insert(2);
    inserted_tree.insert(8);
    inserted_tree.insert(6);
    inserted_tree.insert(9);
    inserted_tree.insert(12);
    let root_node = inserted_tree.root.as_ref().unwrap().clone();
    inserted_tree.right_rotate(root_node);

    inserted_tree.print_tree();


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_compare() {
        // Create a Red-Black Tree manually
        let node_2 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 2,
            parent: None,
            left: None,
            right: None,
        }));
    
        let node_6 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 6,
            parent: None,
            left: None,
            right: None,
        }));
    
        let node_9 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 9,
            parent: None,
            left: None,
            right: None,
        }));
    
        let node_8 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 8,
            parent: None,
            left: Some(Rc::clone(&node_6)),
            right: Some(Rc::clone(&node_9)),
        }));
    
        let node_12 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 12,
            parent: None,
            left: None,
            right: None,
        }));
    
        let node_10 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 10,
            parent: None,
            left: Some(Rc::clone(&node_8)),
            right: Some(Rc::clone(&node_12)),
        }));
    
        let node_5 = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Black,
            key: 5,
            parent: None,
            left: Some(Rc::clone(&node_2)),
            right: Some(Rc::clone(&node_10)),
        }));
    
        //assign parents:
        node_2.borrow_mut().parent = Some(Rc::clone(&node_5));
        node_10.borrow_mut().parent = Some(Rc::clone(&node_5));
    
        node_8.borrow_mut().parent = Some(Rc::clone(&node_10));
        node_12.borrow_mut().parent = Some(Rc::clone(&node_10));
    
        node_6.borrow_mut().parent = Some(Rc::clone(&node_8));
        node_9.borrow_mut().parent = Some(Rc::clone(&node_8));
    
        let mut rb_tree = RedBlackTreeSet::new();
        rb_tree.root = Some(Rc::clone(&node_5));

        // Create a Red-Black Tree using your insert method
        let mut inserted_tree = RedBlackTreeSet::new();
        inserted_tree.insert(10);
        inserted_tree.insert(5);
        inserted_tree.insert(15);
        // Continue inserting nodes...

        // Now you can compare the two trees or assert their equality
        assert_eq!(rb_tree, inserted_tree);
    }
}


