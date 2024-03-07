use std::{cell::RefCell, cmp, rc::Rc};

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    data: T,
    height: i32,
    left: NodePtr<T>,
    right: NodePtr<T>,
}

impl<T: Ord> Node<T> {
    fn new(data: T) -> Self {
        Self {data, height: 1, left: None, right: None}
    }

    fn subtree_height(node: &NodePtr<T>) -> i32 {
        node.as_ref().map_or(0, |node| node.borrow().height)
    }

    fn update_height(&mut self) {
        let left_height = Self::subtree_height(&self.left);
        let right_height = Self::subtree_height(&self.right);
        self.height = 1 + cmp::max(left_height, right_height);
    }

    fn balance_factor(&self) -> i32 {
        let left_height = Self::subtree_height(&self.left);
        let right_height = Self::subtree_height(&self.right);
        left_height - right_height
    }

    pub fn height(&self) -> i32 {
        self.height
    }


    fn rotate_right(root: Rc<RefCell<Self>>) -> NodePtr<T> {
        let new_root = root.borrow_mut().left.take().expect("Left child must exist for right rotation");
        root.borrow_mut().left = new_root.borrow_mut().right.take();
        new_root.borrow_mut().right = Some(root.clone());

        root.borrow_mut().update_height();
        new_root.borrow_mut().update_height();

        Some(new_root)
    }
    

    fn rotate_left(root: Rc<RefCell<Self>>) -> NodePtr<T> {
        let new_root = root.borrow_mut().right.take().expect("Right child must exist for left rotation");
        root.borrow_mut().right = new_root.borrow_mut().left.take();
        new_root.borrow_mut().left = Some(root.clone());

        root.borrow_mut().update_height();
        new_root.borrow_mut().update_height();

        Some(new_root)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(10);
        assert_eq!(node.data, 10); 
        assert_eq!(node.height, 1); 
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_update_height() {
        let mut node = Node::new(10);
        let left_child = Rc::new(RefCell::new(Node::new(5)));
        let right_child = Rc::new(RefCell::new(Node::new(15)));

        node.left = Some(left_child.clone());
        node.right = Some(right_child.clone());

        left_child.borrow_mut().height = 2;
        right_child.borrow_mut().height = 3;

        node.update_height();

        assert_eq!(node.height, 4);
    }

    #[test]
    fn test_balance_factor() {
        let mut node = Node::new(10);
        let left_child = Rc::new(RefCell::new(Node::new(5)));
        let right_child = Rc::new(RefCell::new(Node::new(15)));

        node.left = Some(left_child.clone());
        node.right = Some(right_child.clone());

        left_child.borrow_mut().height = 2;
        right_child.borrow_mut().height = 3;

        node.update_height();

        let balance_factor = node.balance_factor();
        assert_eq!(balance_factor, -1);
    }

    #[test]
    fn test_rotate_right() {
        let root = Rc::new(RefCell::new(Node::new(10)));
        let left_child = Rc::new(RefCell::new(Node::new(5)));
        let left_left_child = Rc::new(RefCell::new(Node::new(2)));

        root.borrow_mut().left = Some(left_child.clone());
        left_child.borrow_mut().left = Some(left_left_child.clone());

        let new_root = Node::rotate_right(root.clone()).unwrap();

        assert_eq!(new_root.borrow().data, 5);
        assert!(new_root.borrow().right.is_some());
        assert_eq!(new_root.borrow().right.as_ref().unwrap().borrow().data, 10);
    }
}
