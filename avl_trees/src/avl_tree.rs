struct AVLTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    fn height(node: &Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        match node.as_ref() {
            Some(rc_node) => {
                rc_node.borrow().height;
            },
            None => {
                0
            }
        }    
    }

    fn update_height(node: &Rc<RefCell<TreeNode<T>>>) {
        let left_height = Self::height(&node.borrow().left);
        let right_height = Self::height(&node.borrow().right);
        node.borrow_mut().height = max(left_height, right_height) + 1;
    }

    fn balance_factor(node: &Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if let Some(ref node) = node {
            let left_height = Self::height(&node.borrow().left);
            let right_height = Self::height(&node.borrow().right);
            left_height - right_height
        } else {
            0
        }
    }

    fn rotate_left(node: Rc<RefCell<TreeNode<T>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let node_right = node.borrow_mut().right.take();
        let node_right_left = node_right.borrow_mut().left.take();
        
        node.borrow_mut().right = node_right_left;
        node_right.borrow_mut().left = Some(node.clone());

        Self::update_height(&node);
        Self::update_height(&node_right);

        Some(node_right)
    }

    fn rotate_right(node: Rc<RefCell<TreeNode<T>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let node_left = node.borrow_mut().left.take();
        let node_left_right = node_left.borrow_mut().right.take();
        
        node.borrow_mut().left = node_left_right;
        node_left.borrow_mut().right = Some(node.clone());

        Self::update_height(&node);
        Self::update_height(&node_left);

        Some(node_left)
    }

}