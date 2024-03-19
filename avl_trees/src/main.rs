use avl_trees::tree::Tree;


fn main() {
    let mut tree: Tree<i32> = Tree::new();

    // Insert elements into the tree
    tree.insert(2);
    tree.insert(5);
    tree.insert(3);
    tree.insert(4);
    tree.insert(17);

    tree.delete(2);
    tree.search(17);
    tree.search(3);
    tree.search(2);
    
    // tree.insert(1);
    // tree.insert(3);
    // tree.insert(4);
    // tree.insert(5);
    // tree.insert(12);
    // tree.insert(17);

    println!("{:#?}", tree);
}