mod node;
mod tree;

fn main() {
    // Create a new tree of integers
    let mut tree = tree::Tree::new();

    // Insert elements into the tree
    tree.insert(2);
    tree.insert(1);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    // tree.insert(12);
    // tree.insert(17);

    println!("{:#?}", tree);
}