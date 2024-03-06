use std::{cell::RefCell, cmp, rc::Rc};

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Ord> {
    value: T,
    height: i32,
    left: NodePtr<T>,
    right: NodePtr<T>
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node{
            value,
            height: 1,
            left: None,
            right: None
        }))
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |left| left.borrow().height);
        let right_height = self.right.as_ref().map_or(0, |right| right.borrow().height);
        self.height = 1 + std::cmp::max(left_height, right_height);
    }

    fn balance_factor(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |left| left.borrow().height);
        let right_height = self.right.as_ref().map_or(0, |right| right.borrow().height);
        left_height - right_height
    }
    

    pub fn get_height(&self) -> i32 {
        self.height
    }


    fn rotate_right(node: Rc<RefCell<Self>>) -> NodePtr<T> {
        let node_left = node.borrow_mut().left.take().expect("Left child must exist for right rotation");
        node.borrow_mut().left = node_left.borrow_mut().right.take();
        node_left.borrow_mut().right = Some(node.clone());

        node.borrow_mut().update_height();
        node_left.borrow_mut().update_height();

        Some(node_left)
    }

    fn rotate_left(node: Rc<RefCell<Self>>) -> NodePtr<T> {
        let node_right = node.borrow_mut().right.take().expect("Right child must exist for left rotation");
        node.borrow_mut().right = node_right.borrow_mut().left.take();
        node_right.borrow_mut().left = Some(node.clone());

        node.borrow_mut().update_height();
        node_right.borrow_mut().update_height();

        Some(node_right)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(10);
        assert_eq!(node.borrow().value, 10);
        assert_eq!(node.borrow().height, 1);
        assert!(node.borrow().left.is_none());
        assert!(node.borrow().right.is_none());
    }

    #[test]
    fn test_update_height() {
        let node = Node::new(10);
        let left_child = Node::new(5);
        let right_child = Node::new(15);

        node.borrow_mut().left = Some(left_child.clone());
        node.borrow_mut().right = Some(right_child.clone());

        // Manually set the height of children to simulate a tree structure
        left_child.borrow_mut().height = 2;
        right_child.borrow_mut().height = 3;

        node.borrow_mut().update_height();

        assert_eq!(node.borrow().height, 4);
    }

    #[test]
    fn test_balance_factor() {
        let node = Node::new(10);
        let left_child = Node::new(5);
        let right_child = Node::new(15);

        node.borrow_mut().left = Some(left_child.clone());
        node.borrow_mut().right = Some(right_child.clone());

        // Manually set the height of children to simulate a tree structure
        left_child.borrow_mut().height = 2;
        right_child.borrow_mut().height = 3;

        // Update parent node height after changing children
        node.borrow_mut().update_height();

        let balance_factor = node.borrow().balance_factor();
        assert_eq!(balance_factor, -1); // Right subtree is taller by 1
    }

    #[test]
    fn test_rotate_right() {
        let root = Node::new(10);
        let left_child = Node::new(5);
        let left_left_child = Node::new(2);

        root.borrow_mut().left = Some(left_child.clone());
        left_child.borrow_mut().left = Some(left_left_child.clone());

        let new_root = Node::rotate_right(root.clone()).unwrap();

        assert_eq!(new_root.borrow().value, 5);
        assert!(new_root.borrow().right.is_some());
        assert_eq!(new_root.borrow().right.as_ref().unwrap().borrow().value, 10); 
    }
}

