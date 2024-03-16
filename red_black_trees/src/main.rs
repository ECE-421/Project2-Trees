
use rbt::RedBlackTreeSet;
fn main() {
    let mut rb_tree_fix = RedBlackTreeSet::new();

    rb_tree_fix.insert(5);
    rb_tree_fix.insert(2);
    rb_tree_fix.insert(10);
    rb_tree_fix.insert(12);
    rb_tree_fix.insert(8);
    rb_tree_fix.insert(6);
    rb_tree_fix.insert(9);
    rb_tree_fix.insert(4);
    rb_tree_fix.insert(13);

    rb_tree_fix.print_tree();
}

mod rbt;