use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

struct TreeNode<T> {
    data: T,
    height: i32,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    fn new(data: T) -> Self {
        Self {data, height: 1, left: None, right: None}
    }
}
