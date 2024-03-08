use std::cell::RefCell;
use std::rc::Rc;

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

impl<T: Ord> TreeNode<T> {
    pub fn new(key: T) -> TreeNode<T> {
        TreeNode {
            color: NodeColor::Black,
            key,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: T) {
        if key < self.key {
            match &mut self.left {
                Some(left_child) => left_child.borrow_mut().insert(key),
                None => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(key)));
                    // need to link parent pointer
                    self.left = Some(new_node);
                }
            }
        } else if key > self.key {
            match &mut self.right {
                Some(right_child) => right_child.borrow_mut().insert(key),
                None => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(key)));
                    // need to link parent pointer
                    self.right = Some(new_node);
                }
            }
        }
    }
}

fn main() {
    // Create a sample Red-Black Tree
    let mut root = Rc::new(RefCell::new(TreeNode::new(10)));
    root.borrow_mut().insert(5);
    root.borrow_mut().insert(15);
    root.borrow_mut().insert(3);
    root.borrow_mut().insert(7);

    println!("Red-Black Tree: {:#?}", root);
}

