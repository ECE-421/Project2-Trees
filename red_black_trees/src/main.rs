
use rbt::RedBlackTreeSet;
fn main() {
    let mut rb_tree_fix = RedBlackTreeSet::new();

    println!("Tree is empty: {:?}", rb_tree_fix.is_empty());

    rb_tree_fix.insert(5);
    rb_tree_fix.insert(2);
    rb_tree_fix.insert(10);
    rb_tree_fix.insert(12);
    rb_tree_fix.insert(8);
    rb_tree_fix.insert(6);
    rb_tree_fix.insert(9);
    rb_tree_fix.insert(4);
    rb_tree_fix.insert(13);
    // rb_tree_fix.insert(15);
    
    rb_tree_fix.print_tree();

    println!("Num leaves: {:?}", rb_tree_fix.leaves());
    println!("Tree is empty: {:?}", rb_tree_fix.is_empty());
    println!("Tree height: {:?}",  rb_tree_fix.get_tree_height());

    println!("Inorder print of tree:");
    rb_tree_fix.print_in_order_traversal();

}

mod rbt;