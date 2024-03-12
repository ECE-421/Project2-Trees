
use rbt::RedBlackTreeSet;
fn main() {
    let mut rb_tree = RedBlackTreeSet::new();
    rb_tree.insert(5);
    rb_tree.insert(2);
    rb_tree.insert(10);
    rb_tree.insert(8);
    rb_tree.insert(6);
    rb_tree.insert(9);
    rb_tree.insert(12);

    // Print the tree before right rotation
    println!("Before Right Rotation:");
    rb_tree.print_tree();

    // Find the node you want to rotate (for example, the root in this case)
    let root_node = rb_tree.root.as_ref().unwrap().clone();

    // Perform right rotation
    rb_tree.left_rotate(root_node);

    // Print the tree after right rotation
    println!("After Right Rotation:");
    rb_tree.print_tree();


    // // Create a Red-Black Tree manually
    // let node_2 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 2,
    //     parent: None,
    //     left: None,
    //     right: None,
    // }));

    // let node_6 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 6,
    //     parent: None,
    //     left: None,
    //     right: None,
    // }));

    // let node_9 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 9,
    //     parent: None,
    //     left: None,
    //     right: None,
    // }));

    // let node_8 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 8,
    //     parent: None,
    //     left: Some(Rc::clone(&node_6)),
    //     right: Some(Rc::clone(&node_9)),
    // }));

    // let node_12 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 12,
    //     parent: None,
    //     left: None,
    //     right: None,
    // }));

    // let node_10 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 10,
    //     parent: None,
    //     left: Some(Rc::clone(&node_8)),
    //     right: Some(Rc::clone(&node_12)),
    // }));

    // let node_5 = Rc::new(RefCell::new(TreeNode {
    //     color: NodeColor::Black,
    //     key: 5,
    //     parent: None,
    //     left: Some(Rc::clone(&node_2)),
    //     right: Some(Rc::clone(&node_10)),
    // }));

    // //assign parents:
    // node_2.borrow_mut().parent = Some(Rc::clone(&node_5));
    // node_10.borrow_mut().parent = Some(Rc::clone(&node_5));

    // node_8.borrow_mut().parent = Some(Rc::clone(&node_10));
    // node_12.borrow_mut().parent = Some(Rc::clone(&node_10));

    // node_6.borrow_mut().parent = Some(Rc::clone(&node_8));
    // node_9.borrow_mut().parent = Some(Rc::clone(&node_8));

    // let mut rb_tree = RedBlackTreeSet::new();
    // rb_tree.root = Some(Rc::clone(&node_5));
    // rb_tree.print_tree();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_compare() {
        
    }
}


mod rbt;